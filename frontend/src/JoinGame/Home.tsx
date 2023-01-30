import React from "react";
import { useNavigate } from "react-router-dom";
type HomeProps = {
    username: string,
    sendName: (name: string) => void;
}

export default function Home(props: HomeProps) {
    const handleChange = (e: any) => {
        props.sendName(e.target.value);
    }
    const navigate = useNavigate();
    const joinGame = async (_e: any) => {
        let ws = new WebSocket(`ws://localhost:9090/join-game?username=${props.username}`);
        ws.onopen = () => {
            console.log("Websocket has opened!");
        }
        ws.onmessage = (msg: any) => {
            console.log(JSON.parse(msg.data));
        }
        navigate("/lobby");
    }
    return (
        <div className='background flex'>
            <button className='joinButton center among-us-join' onClick={joinGame}>
                Join Game
            </button>
            <div className='flex'>
                <label className='user-label' htmlFor="name">Username:</label>
                <input onChange={handleChange} className='user-input'
                    type="text" defaultValue={props.username}
                    required minLength={1} maxLength={10} size={12}></input>
            </div>
        </div>
    );
}

