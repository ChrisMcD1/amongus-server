import { useState } from "react";
import { ReactComponent as AmongusMan } from "./Whitetest.svg";
import { BlockPicker, ColorResult } from "react-color";
import { useNavigate } from "react-router-dom";
type LobbyProps = { username: string; ws: WebSocket | undefined; };
import Color from "color";
import { useAppDispatch } from '../hooks'
import { selectCurrentPlayer, selectOtherPlayers, setPlayerColor } from "../state/playersSlice";
import { useSelector } from "react-redux";
import PlayerTile from "../InGame/PlayerTile";
import { createColorMessage } from "../Messages/toServer";


export default function Lobby(props: LobbyProps) {
    const [background, setBackground] = useState("#000000");
    const [check, setCheck] = useState(false);

    const navigate = useNavigate();
    const dispatch = useAppDispatch();

    const otherPlayers = useSelector(selectOtherPlayers)

    const currentPlayer = useSelector(selectCurrentPlayer);
    console.log("current player color is :", currentPlayer?.color);

    const playerColor = currentPlayer?.color ?? "#FF00FF";
    let darkerColor = Color(playerColor).darken(0.3);
    document.documentElement.style.setProperty("--base-color", playerColor);
    document.documentElement.style.setProperty("--shadow-color", darkerColor.hex());


    const handleChange = (color: ColorResult) => {
        setBackground(color.hex);
        dispatch(setPlayerColor({ color: color.hex, id: currentPlayer!.id }));
        if (props.ws) {
            props.ws.send(createColorMessage(color.hex));
        }
    };

    return (
        <div className="h-screen w-screen items-center bg-lobby bg-cover bg-center">
            <div className="flex flex-col items-center">
                <h3 className="mx-auto absolute font-amongus-log top-[9rem] md:top-[20rem] md:text-lg text-white">{currentPlayer?.username}</h3>
                <AmongusMan
                    className="player absolute inset-1/4 top-[27%] mx-auto h-12 items-center md:h-20"
                    onClick={() => setCheck(!check)}
                />
                <div
                    className="absolute top-1/3"
                    style={{ display: check ? "initial" : "none" }}
                >
                    <BlockPicker color={background} onChange={handleChange} />
                </div>
            </div>
            <div className="absolute bottom-0">
                {otherPlayers.map((player) => (
                    <PlayerTile key={player.id} {...player} />
                ))
                }
            </div>
        </div>
    );
}
