import { useState } from "react";
import whiteTest from './Whitetest.svg';
import { BlockPicker, ColorResult } from 'react-color';
import start from "./start.png"

type LobbyProps = { username: string }

export default function Lobby(props: LobbyProps) {
    const [background, setBackground] = useState("#000000");
    const [check, setCheck] = useState(false);

    const handleChange = (color: ColorResult) => {
        setBackground(color.hex);
        document.documentElement.style.setProperty('--base-color', color.hex);
        document.documentElement.style.setProperty('--shadow-color', color.hex);
    }

    const myFunction = () => {
        console.log("CLICKED");
    }

    return (
        <div className='bg-lobby bg-center h-screen w-screen bg-cover'>
            <div className='flex flex-col justify-center'>
                <h3 className='text-white mx-auto mt-32'>{props.username}</h3>
                <img src={whiteTest} className="h-32 player mx-auto fill-current-[#222] border-black text-[#222]" onClick={() => setCheck(!check)} />
                <button style={{ display: check ? "none" : "initial" }}
                    className='bg-transparent mx-auto'>
                    <img src={start} onClick={myFunction} />
                </button>
            </div>
            <div style={{ display: check ? "initial" : "none" }} className={'colorPick center'}>
                <BlockPicker color={background} onChange={handleChange} />
            </div>
        </div>
    );
}
