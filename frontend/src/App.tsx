import React from "react";
import Lobby from './JoinGame/Lobby';
import Home from './JoinGame/Home';
import Crewmate from './InGame/Crewmate'
import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route }
    from 'react-router-dom';


export default function App() {
    const [username, setUsername] = useState('')

    function handleChangeName(newName: string) {
        setUsername(newName);
    }

    return (
        <Router>
            <Routes> <Route path='/lobby' element={<Lobby username={username} />} />
                <Route path='/' element={<Home sendName={handleChangeName} />} />
                <Route path='/crewmate' element={<Crewmate />} /></Routes>
        </Router>
    );
}
