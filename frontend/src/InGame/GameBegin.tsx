import React, { useState } from "react";
type HomeProps = {
    sendName: Function;
}

export default function GameBegin(props: HomeProps) {
    const [username, _setUsername] = useState("");
    const handleChange = (e: any) => {
        props.sendName(e.target.value);
    }

    return (
        <div className='background flex'>
            <div className='flex'>
                <label className='user-label' htmlFor="name">Username:</label>
                <input onChange={handleChange} className='user-input'
                    type="text" defaultValue={username}
                    required minLength={1} maxLength={10} size={12}></input>
            </div>
        </div>
    );
}

