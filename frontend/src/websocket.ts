import { deleteAllPlayers, setPlayerRole, updatePlayerStatus } from './state/playersSlice';
import { showErrorMessage, hideErrorMessage } from './state/errorsSlice';
import store from './state/store';
import { ChatMessage, GameState, PlayerStatus, PreZodMessage, SetRole, Winner, InvalidAction, EmergencyMeetingCalled } from "./Messages/fromServer";
import { selectCurrentPlayerID, setUserID } from './state/userSlice';
import { push } from 'redux-first-history';
import { beginEmergencyMeeting } from './state/meetingSlice';


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
            store.dispatch(updatePlayerStatus(playerStatus))
            break;
        }
        case "GameOver": {
            const winner = Winner.parse(parsed.content);
            switch (winner) {
                case "imposters": {
                    store.dispatch(push("/imposter-victory"))
                    break;
                }
                case "crewmates": {
                    throw new Error("Not implemented")
                }
                default: {
                    throw new Error("Unreachable");
                }
            }
            break;
        }
        case "InvalidAction": {
            const invalidAction = InvalidAction.parse(parsed.content);
            store.dispatch(showErrorMessage(invalidAction));
            setTimeout(() => {
                store.dispatch(hideErrorMessage())
            }, 5000)
            break;
        }
        case "EmergencyMeetingCalled": {
            const emergencyMeetingCalled = EmergencyMeetingCalled.parse(parsed.content);
            store.dispatch(beginEmergencyMeeting(emergencyMeetingCalled.initiator));
            store.dispatch(push("/meeting"));
            break;
        }
        default: {
            console.warn(`Got nonconfigured message of type: ${parsed.type}, and message:`, parsed.content);
        }

    }
}

