import Lobby from './JoinGame/Lobby';
import Home from './JoinGame/Home';
import Crewmate from './InGame/Crewmate'
import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route }
    from 'react-router-dom';
import StatusOverview from './JoinGame/StatusOverview';


export default function App() {
    const [username, setUsername] = useState('')
    const [ws, setWs] = useState<WebSocket>();

    function handleChangeName(newName: string) {
        setUsername(newName);
        let websocket = new WebSocket(`localhost:8080/join-game?username=${newName}`);
        websocket.onmessage = ((msg: MessageEvent) => console.log(msg.data));
        setWs(websocket);
    }

    return (
        <Router>
            <Routes>
                <Route path='/lobby' element={<Lobby username={username} />} />
                <Route path='/' element={<Home username={username} sendName={handleChangeName} />} />
                <Route path='/crewmate' element={<Crewmate />} />
                <Route path='/status-overview' element={<StatusOverview ws={ws} />} />
            </Routes>
        </Router>
    );
}
