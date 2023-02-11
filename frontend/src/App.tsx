import Lobby from './JoinGame/Lobby';
import Home from './JoinGame/Home';
import Crewmate from './InGame/Crewmate'
import { useState } from 'react';
import { BrowserRouter as Router, Routes, Route }
    from 'react-router-dom';
import StatusOverview from './InGame/StatusOverview';


export default function App() {
    const [username, setUsername] = useState('')
    const [ws, setWs] = useState<WebSocket>();

    const player_id =
        document.cookie
            .split('; ')
            .find((row) => row.startsWith('player_id='))
            ?.split('=')[1];
    console.log(`Player has id: ${player_id}`);


    function handleChangeName(newName: string) {
        setUsername(newName);
    }

    return (
        <Router>
            <Routes>
                <Route path='/lobby' element={<Lobby username={username} />} />
                <Route path='/' element={<Home username={username} setWs={setWs} sendName={handleChangeName} />} />
                <Route path='/crewmate' element={<Crewmate />} />
                <Route path='/status-overview' element={<StatusOverview ws={ws} />} />
            </Routes>
        </Router>
    );
}
