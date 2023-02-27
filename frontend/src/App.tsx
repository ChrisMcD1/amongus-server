import Lobby from "./JoinGame/Lobby";
import Home from "./JoinGame/Home";
import Crewmate from "./InGame/Crewmate";
import { useState } from "react";
import {
    Routes,
    Route,
} from "react-router-dom";
import {
    HistoryRouter as Router
} from "redux-first-history/rr6";
import StatusOverview from "./InGame/StatusOverview";
import Dashboard from "./InGame/Dashboard";
import { configureWebsocket } from "./websocket";
import GameBegin from "./InGame/GameBegin";
import { Provider } from "react-redux";
import { store, history } from "./state/store";
import Admin from "./Admin/Admin";
import { setUserID } from "./state/userSlice";
import ImposterVictory from "./InGame/ImposterVictory";

export default function App() {
    const [username, setUsername] = useState("");
    const [ws, setWs] = useState<WebSocket>();

    const player_id = document.cookie
        .split("; ")
        .find((row) => row.startsWith("player_id="))
        ?.split("=")[1];
    console.log(`Player has id: ${player_id}`);

    if (player_id != null) {
        store.dispatch(setUserID(player_id));
    }


    if (player_id != null && ws == null) {
        fetch(`http://localhost:9090/player-exists?id=${player_id}`)
            .then((res) => res.json())
            .then((exists) => {
                console.log(exists);
                if (exists) {
                    let ws = new WebSocket(
                        `ws://localhost:9090/player-rejoin-game?id=${player_id}`
                    );
                    ws = configureWebsocket(ws);
                    console.log(ws);
                    setWs(ws);

                    if (player_id != null) {
                        store.dispatch(setUserID(player_id));
                    }
                } else {
                    document.cookie =
                        "player_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
                }
            });
    }

    return (
        <Provider store={store}>
            <Router history={history}>
                <Routes>
                    <Route
                        path="/lobby"
                        element={<Lobby username={username} ws={ws} />}
                    />
                    <Route
                        path="/"
                        element={
                            <Home
                                username={username}
                                setWs={setWs}
                                setUsername={setUsername}
                            />
                        }
                    />
                    <Route path="/crewmate" element={<Crewmate username={username} />} />
                    <Route path="/begin" element={<GameBegin username={username} />} />
                    <Route path="/dashboard" element={<Dashboard />} />
                    <Route path="/status-overview" element={<StatusOverview ws={ws} />} />
                    <Route path="/admin" element={<Admin />} />
                    <Route path="/imposter-victory" element={<ImposterVictory />} />
                </Routes>
            </Router>
        </Provider>
    );
}
