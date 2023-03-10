import { deleteAllPlayers, setPlayerDead, setPlayerRole, updatePlayerStatus } from './state/playersSlice';
import { showErrorMessage, hideErrorMessage } from './state/errorsSlice';
import store from './state/store';
import { ChatMessage, GameState, PlayerStatus, PreZodMessage, SetRole, Winner, InvalidAction, EmergencyMeetingCalled, BodyReported, PlayerDied } from "./Messages/fromServer";
import { setUserID } from './state/userSlice';
import { push } from 'redux-first-history';
import { beginEmergencyMeeting, beginReportedBodyMeeting } from './state/meetingSlice';


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
            let playerRole = SetRole.parse(parsed.content);
            store.dispatch(setPlayerRole(playerRole))
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
                    store.dispatch(push("/crewmate-victory"))
                    break;
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
        case "BodyReported": {
            const bodyReported = BodyReported.parse(parsed.content);
            store.dispatch(beginReportedBodyMeeting(bodyReported));
            store.dispatch(push("/meeting"));
            break;
        }
        case "SuccessfulKill": {
            store.dispatch(setPlayerDead(parsed.content));
            break;
        }
        case "PlayerDied": {
            let _playerDied = PlayerDied.parse(parsed.content);
            store.dispatch(setPlayerDead(
                store.getState().user.id
            ));
            break;
        }
        default: {
            console.warn(`Got nonconfigured message of type: ${parsed.type}, and message:`, parsed.content);
        }

    }
}

