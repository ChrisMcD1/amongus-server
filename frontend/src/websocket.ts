import { changeColor } from "./JoinGame/colorSlice";
import { addPlayer } from './playersSlice';
import { setUser } from "./JoinGame/userSlice";
import Color from "color";
import store from './store';
import { PlayerStatus, PreZodMessage } from "./Messages/fromServer";


export function configureWebsocket(ws: WebSocket): WebSocket {
    ws.onopen = () => {
        console.log("Websocket has opened!");
    }
    ws.onmessage = processWebsocketMessage;
    return ws;
}


function processWebsocketMessage(msg: MessageEvent<any>) {
    const globalPlayers = store.getState().players;
    const dispatch = store.dispatch;
    const color = store.getState().color.color;
    const user = store.getState().user.user;

    // if (useAppSelector == null) {
    //     console.error("Got websocket message before hook was set up. BAD");
    //     return;
    // }
    let parsed = JSON.parse(msg.data) as PreZodMessage;
    switch (parsed.type) {
        case "PlayerStatus": {
            let playerStatus = PlayerStatus.parse(parsed.content);
            console.log(`parsed player status to ${JSON.stringify(playerStatus)}`);
            let existingPlayer = globalPlayers?.players.find(player => player.id === playerStatus.id);
            if (existingPlayer == null) {
                dispatch(addPlayer({
                    role: null,
                    color: color,
                    id: playerStatus.id,
                    name: user,
                    alive: true
                }))
            } else {
                dispatch(changeColor(playerStatus.color));
                dispatch(setUser(playerStatus.username));
                let color = store.getState().color.color;
                let darkerColor = Color(color).darken(0.3);
                document.documentElement.style.setProperty(
                    "--base-color",
                    store.getState().color.color
                );
                document.documentElement.style.setProperty(
                    "--shadow-color",
                    darkerColor.hex()
                );
            }
            break;
        }
        default: {
            console.warn(`Got nonconfigured message of type: ${parsed.type}, and message:`, parsed.content);
        }

    }
}

