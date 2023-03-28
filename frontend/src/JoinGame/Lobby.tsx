import { useState } from "react";
import { ReactComponent as AmongusMan } from "./Whitetest.svg";
import { BlockPicker, ColorResult } from "react-color";
import { useNavigate } from "react-router-dom";
type LobbyProps = { username: string; ws: WebSocket | undefined };
import Color from "color";
import { useAppDispatch } from "../hooks";
import {
  selectCurrentPlayer,
  selectOtherPlayers,
  setPlayerColor,
} from "../state/playersSlice";
import { useSelector } from "react-redux";
import PlayerTile from "../InGame/PlayerTile";
import { createColorMessage } from "../Messages/toServer";

export default function Lobby(props: LobbyProps) {
  const [background, setBackground] = useState("#000000");
  const [check, setCheck] = useState(false);

  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  const otherPlayers = useSelector(selectOtherPlayers);

  const currentPlayer = useSelector(selectCurrentPlayer);
  console.log("current player color is :", currentPlayer?.color);

  const playerColor = currentPlayer?.color ?? "#FF00FF";
  let darkerColor = Color(playerColor).darken(0.3);
  //    document.documentElement.style.setProperty("--base-color", playerColor);
  //    document.documentElement.style.setProperty("--shadow-color", darkerColor.hex());

  const handleChange = (color: ColorResult) => {
    setBackground(color.hex);
    dispatch(setPlayerColor({ color: color.hex, id: currentPlayer!.id }));
    if (props.ws) {
      props.ws.send(createColorMessage(color.hex));
    }
  };

  return (
    <div className="h-screen w-screen items-center bg-black bg-lobby bg-[length:auto_100%] bg-center bg-no-repeat">
      <div className="flex h-[100%] flex-col items-center">
        <div className="absolute top-[20%] h-[12%]">
          <h3 className="mx-auto text-center font-amongus-log text-2xl text-white">
            {currentPlayer?.username}
          </h3>
          <AmongusMan
            style={{
              ["--base-color" as any]: playerColor,
              ["--shadow-color" as any]: Color(playerColor).darken(0.3),
            }}
            className="mx-auto h-[100%] items-center"
            onClick={() => setCheck(!check)}
          />
        </div>
        <div
          className="absolute top-1/3"
          style={{ display: check ? "initial" : "none" }}
        >
          <BlockPicker color={background} onChange={handleChange} />
        </div>
      </div>
      <div className="absolute bottom-0 flex">
        {otherPlayers.map((player) => (
          <PlayerTile
            className="h-32"
            key={player.id}
            {...player}
            showBorder={false}
            isSelected={false}
          />
        ))}
      </div>
    </div>
  );
}
