import { deleteAllPlayers, selectCurrentPlayer, setPlayerRole, updatePlayerStatus } from './state/playersSlice';
import store from './state/store';
import { ChatMessage, GameState, PlayerStatus, PreZodMessage, RoleAssignment, SetRole } from "./Messages/fromServer";
import z from "zod";
import { selectCurrentPlayerID, setUserID } from './state/userSlice';
import { push } from 'redux-first-history';


export function configureWebsocket(ws: WebSocket): WebSocket {
    ws.onopen = () => {
        console.log("Websocket has opened!");
    }
    ws.onmessage = processWebsocketMessage;
    return ws;
}


function processWebsocketMessage(msg: MessageEvent<any>) {

    let parsed = JSON.parse(msg.data) as PreZodMessage;
    switch (parsed.type) {
        case "ChatMessage": {
            let chatMessage = ChatMessage.parse(parsed.content);
            break;
        }
        case "PlayerRole": {
            let currentPlayerID = selectCurrentPlayerID(store.getState());
            let playerRole = SetRole.parse(parsed.content);
            store.dispatch(setPlayerRole({
                id: currentPlayerID,
                role: playerRole.role
            }))
            break;
        }
        case "AssignedID": {
            store.dispatch(setUserID(parsed.content));
            break;
        }
        case "GameState": {
            const gameState = GameState.parse(parsed.content);
            switch (gameState.state) {
                case "lobby": {
                    break;
                }
                case "inGame": {
                    store.dispatch(push("/status-overview"));
                    break;
                }
                case "reset": {
                    store.dispatch(push("/"));
                    store.dispatch(deleteAllPlayers());
                    break;
                }
                default: {
                    throw new Error("Received unknown game state!");
                }

            }
            break;
        }
        case "PlayerStatus": {
            let playerStatus = PlayerStatus.parse(parsed.content);
            handlePlayerStatusMessage(playerStatus);
            break;
        }
        default: {
            console.warn(`Got nonconfigured message of type: ${parsed.type}, and message:`, parsed.content);
        }

    }
}

function handlePlayerStatusMessage(playerStatus: z.infer<typeof PlayerStatus>) {
    const dispatch = store.dispatch;
    dispatch(updatePlayerStatus(playerStatus))
}

//function handleChatMessageMessage(chatMessage: z.infer<typeof ChatMessage>) {
//    store.dispatch();
//}
