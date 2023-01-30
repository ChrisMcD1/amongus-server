import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
type HomeProps = {
    sendName: (name: string) => void;
}

export default function Home(props: HomeProps) {
    const [username, _setUsername] = useState("");
    const handleChange = (e: any) => {
        props.sendName(e.target.value);
    }
    const navigate = useNavigate();
    const joinGame = (_e: any) => {
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
                    type="text" defaultValue={username}
                    required minLength={1} maxLength={10} size={12}></input>
            </div>
        </div>
    );
}

