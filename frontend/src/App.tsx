import Lobby from "./JoinGame/Lobby";
import Home from "./JoinGame/Home";
import Crewmate from "./InGame/Crewmate";
import { useState } from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import StatusOverview, { Player } from "./InGame/StatusOverview";
import Dashboard from "./InGame/Dashboard";
import { configureWebsocket } from "./websocket";
import React from "react";
import GameBegin from "./InGame/GameBegin";

export type PlayersContextType = {
  ws: WebSocket | undefined;
  players: Array<Player>;
  setPlayers: (players: Array<Player>) => void;
};

export const PlayersContext = React.createContext<PlayersContextType | null>(
  null
);

export default function App() {
  const [username, setUsername] = useState("");
  const [ws, setWs] = useState<WebSocket>();
  const [players, setPlayers] = useState<Array<Player>>([]);

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
          ws = configureWebsocket(ws, { ws, players, setPlayers });
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
      <PlayersContext.Provider
        value={{ players: players, setPlayers: setPlayers, ws: ws }}
      >
        <Routes>
          <Route path="/lobby" element={<Lobby username={username} />} />
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
          <Route path="/crewmate" element={<Crewmate />} />
          <Route path="/begin" element={<GameBegin />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/status-overview" element={<StatusOverview />} />
        </Routes>
      </PlayersContext.Provider>
    </Router>
  );
}
