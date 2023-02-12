import { z } from "zod";
import { PlayersContextType } from "./App";

let globalPlayers: PlayersContextType | null = null;


export function configureWebsocket(ws: WebSocket, playersContext: PlayersContextType): WebSocket {
    ws.onopen = () => {
        console.log("Websocket has opened!");
    }
    globalPlayers = playersContext;
    ws.onmessage = processWebsocketMessage;
    return ws;
}


function processWebsocketMessage(msg: MessageEvent<any>) {
    if (globalPlayers == null) {
        console.error("Got websocket message before hook was set up. BAD");
        return;
    }
    let parsed = JSON.parse(msg.data);
    console.log(parsed);
    let [type, contents] = Object.entries(parsed)[0];
    switch (type) {
        case "PlayerStatus": {
            let playerStatus = PlayerStatus.parse(contents);
            console.log(`parsed player status to ${JSON.stringify(playerStatus)}`);
            let existingPlayer = globalPlayers?.players.find(player => player.id === playerStatus.id);
            if (existingPlayer == null) {
                globalPlayers.setPlayers([...globalPlayers.players, {
                    color: "#aaa",
                    id: playerStatus.id,
                    name: playerStatus.username,
                    alive: true
                }])
            }

        }
        default: {
            console.warn(`Got nonconfigured message of type: ${type}, and message:`, contents);
        }

    }
}

const PlayerConnectionStatus = z.enum(["new", "disconnected", "reconnected", "existing"]);

const PlayerStatus = z.object({
    username: z.string(),
    id: z.string(),
    status: PlayerConnectionStatus
})
