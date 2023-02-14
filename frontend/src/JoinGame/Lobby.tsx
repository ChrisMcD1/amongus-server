import { useState } from "react";
import { ReactComponent as Whitetest } from "./Whitetest.svg";
import { BlockPicker, ColorResult } from "react-color";
import start from "./start.png";
import { useNavigate } from "react-router-dom";
type LobbyProps = { username: string; ws: WebSocket | undefined; };
import Color from "color";
import { useAppSelector, useAppDispatch } from '../hooks'
import { changeColor } from './colorSlice'
import { setPlayerColor } from "../playersSlice";


export default function Lobby(props: LobbyProps) {
    const [background, setBackground] = useState("#000000");
    const [check, setCheck] = useState(false);

    const navigate = useNavigate();
    const dispatch = useAppDispatch();

    const handleChange = (color: ColorResult) => {
        setBackground(color.hex);
        dispatch(changeColor(color.hex));
        dispatch(setPlayerColor(color.hex));
        let darkerColor = Color(color.hex).darken(0.3);
        document.documentElement.style.setProperty("--base-color", color.hex);
        document.documentElement.style.setProperty("--shadow-color", darkerColor.hex());
        let colorMsg = {
            type: "ChooseColor",
            content: {
                color: color.hex
            }
        };
        if (props.ws) {
            props.ws.send(JSON.stringify(colorMsg));
        }
    };

    const startGame = () => {
        fetch("http://localhost:9090/start-game", { method: "POST" });
        navigate("/begin");
        setTimeout(() => navigate("/role"), 2000);
        setTimeout(() => navigate("/status-overview"), 4000);
    };

    return (
        <div className="h-screen w-screen items-center bg-lobby bg-cover bg-center">
            <div className="flex flex-col items-center">
                <h3 className="mx-auto absolute font-amongus-log top-[9rem] md:top-[20rem] md:text-lg text-white">{useAppSelector((state) => state.user.user)}</h3>
                <Whitetest
                    className="player absolute inset-1/4 top-[27%] mx-auto h-12 items-center md:h-20"
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
