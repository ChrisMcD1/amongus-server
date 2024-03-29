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
import EmergencyButton from "./InGame/EmergencyButton";
import { configureWebsocket } from "./websocket";
import GameBegin from "./InGame/GameBegin";
import { Provider } from "react-redux";
import { store, history } from "./state/store";
import Dashboard from "./Dashboard/Dashboard";
import { setUserID } from "./state/userSlice";
import ImposterVictory from "./InGame/ImposterVictory";
import ErrorMessage from "./ErrorMessage";
import Meeting from "./InGame/Meeting";
import CrewmateVictory from "./InGame/CrewmateVictory";
import { push } from "redux-first-history";

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
        fetch(`https://${import.meta.env.VITE_BACKEND_SERVER}/player-exists?id=${player_id}`)
            .then((res) => res.json())
            .then((exists) => {
                console.log(exists);
                if (exists) {
                    let ws = new WebSocket(
                        `wss://${import.meta.env.VITE_BACKEND_SERVER}/player-rejoin-game?id=${player_id}`
                    );
                    ws = configureWebsocket(ws);
                    console.log(ws);
                    setWs(ws);

                    if (player_id != null) {
                        store.dispatch(setUserID(player_id));
                    }
                } else {
                }
            });
    }

    if (player_id == null && !window.location.pathname.includes("dashboard")) {
        store.dispatch(push("/"));
    }

    return (
        <Provider store={store}>
            <div>
                <ErrorMessage />
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
                        <Route path="/begin" element={<GameBegin />} />
                        <Route path="/dashboard" element={<Dashboard />} />
                        <Route path="/emergency-button" element={<EmergencyButton />} />
                        <Route path="/meeting" element={<Meeting ws={ws} />} />
                        <Route path="/status-overview" element={<StatusOverview ws={ws} />} />
                        <Route path="/imposter-victory" element={<ImposterVictory />} />
                        <Route path="/crewmate-victory" element={<CrewmateVictory />} />
                    </Routes>
                </Router>
            </div>
        </Provider>
    );
}
