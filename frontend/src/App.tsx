import Lobby from "./JoinGame/Lobby";
import Home from "./JoinGame/Home";
import Crewmate from "./InGame/Crewmate";
import RoleAssignment from "./InGame/RoleAssignment";
import { useState } from "react";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import StatusOverview, { Player } from "./InGame/StatusOverview";
import Dashboard from "./InGame/Dashboard";
import { configureWebsocket } from "./websocket";
import React from "react";
import GameBegin from "./InGame/GameBegin";
import { Provider } from "react-redux";
import store from "./store";
import { changeColor } from "./JoinGame/colorSlice";
import { setUser } from "./JoinGame/userSlice";
import Color from "color";

export default function App() {
  const [username, setUsername] = useState("");
  const [ws, setWs] = useState<WebSocket>();

  const player_id = document.cookie
    .split("; ")
    .find((row) => row.startsWith("player_id="))
    ?.split("=")[1];
  console.log(`Player has id: ${player_id}`);

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
        } else {
          document.cookie =
            "player_id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        }
      });
  }

  return (
    <Router>
      <Provider store={store}>
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
          <Route path="/status-overview" element={<StatusOverview />} />
        </Routes>
      </Provider>
    </Router>
  );
}
