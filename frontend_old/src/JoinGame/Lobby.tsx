import React, { useState } from "react";
import { ReactComponent as White } from './Whitetest.svg';
import { BlockPicker, ColorChangeHandler, ColorResult } from 'react-color';
//@ts-ignore
import { LightenDarkenColor } from 'lighten-darken-color';
import start from "./start.png"

type LobbyProps = { username: string }

export default function Lobby(props: LobbyProps) {
    const [background, setBackground] = useState("#000000");
    const [check, setCheck] = useState(false);

    const handleChange = (color: ColorResult) => {
        setBackground(color.hex);
        document.documentElement.style.setProperty('--base-color', color.hex);
        document.documentElement.style.setProperty('--shadow-color', LightenDarkenColor(color.hex, -35));
    }

    const myFunction = () => {
        console.log("CLICKED");
    }

    return (
        <div className='lobby-background'>
            <div className='player-box center'>
                <h3 className='player-text'>{props.username}</h3>
                <White className="player" onClick={() => setCheck(!check)} />
                <button style={{ display: check ? "none" : "initial" }}
                    className="start-button">
                    <img src={start} onClick={myFunction} />
                </button>
            </div>
            <div style={{ display: check ? "initial" : "none" }} className={'colorPick center'}>
                <BlockPicker color={background} onChange={handleChange} />
            </div>
        </div>
    );
}
