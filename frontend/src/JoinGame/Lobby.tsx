import { useState } from "react";
import whiteTest from "./Whitetest.svg";
import { BlockPicker, ColorResult } from "react-color";
import start from "./start.png";
import { useNavigate } from "react-router-dom";
type LobbyProps = { username: string };

export default function Lobby(props: LobbyProps) {
  const [background, setBackground] = useState("#000000");
  const [check, setCheck] = useState(false);

  const navigate = useNavigate();

  const amogus = {
    fill: "#FF0000",
    stroke: "#FF0000"
  }

  const handleChange = (color: ColorResult) => {
    setBackground(color.hex);
    document.documentElement.style.setProperty("--base-color", color.hex);
    document.documentElement.style.setProperty("--shadow-color", color.hex);
  };

  const startGame = () => {
    fetch("http://localhost:9090/start-game", { method: "POST" });
    navigate("/status-overview");
  };

  return (
    <div className="h-screen w-screen items-center bg-lobby bg-cover bg-center">
      <div className="flex flex-col items-center">
        <h3 className="mx-auto absolute font-amongus-log inset-y-1/4 text-white">{props.username}</h3>
        <img
          src={whiteTest}
          style = {amogus}
          className="player absolute inset-1/4 mx-auto h-12 items-center border-black text-[#222] md:h-20"
          onClick={() => setCheck(!check)}
        />
        <button
          style={{ display: check ? "none" : "initial" }}
          className="fixed top-1/2 mx-auto bg-transparent"
        >
          <img src={start} onClick={startGame} />
        </button>
        <div
          className="absolute top-1/3"
          style={{ display: check ? "initial" : "none" }}
        >
          <BlockPicker color={background} onChange={handleChange} />
        </div>
      </div>
    </div>
  );
}
