import { deleteAllPlayers, setPlayerDead, setPlayerRole, updatePlayerStatus } from './state/playersSlice';
import { showErrorMessage, hideErrorMessage } from './state/errorsSlice';
import store from './state/store';
import { ChatMessage, GameState, PlayerStatus, PreZodMessage, SetRole, Winner, InvalidAction, EmergencyMeetingCalled, BodyReported, PlayerDied, GameStateEnum, MeetingReasonEnum, MeetingReason } from "./Messages/fromServer";
import { setUserID } from './state/userSlice';
import { push } from 'redux-first-history';
import { beginEmergencyMeeting, beginReportedBodyMeeting } from './state/meetingSlice';
import z from "zod";


export function configureWebsocket(ws: WebSocket): WebSocket {
    ws.onopen = () => {
        console.log("Websocket has opened!");
    }
    ws.onmessage = processWebsocketMessage;
    return ws;
}


function processWebsocketMessage(msg: MessageEvent<any>) {

    let parsed = PreZodMessage.parse(JSON.parse(msg.data));
    switch (parsed.type) {
        case "chatMessage": {
            let chatMessage = ChatMessage.parse(parsed.content);
            break;
        }
        case "playerRole": {
            let playerRole = SetRole.parse(parsed.content);
            store.dispatch(setPlayerRole(playerRole))
            break;
        }
        case "assignedID": {
            let userID = z.string().parse(parsed.content);
            store.dispatch(setUserID(userID));
            break;
        }
        case "gameState": {
            let gameState = GameState.parse(parsed.content);
            const gameStateEnum = GameStateEnum.parse(gameState.type)
            switch (gameStateEnum) {
                case "lobby": {
                    store.dispatch(push("/lobby"));
                    break;
                }
                case "inGame": {
                    store.dispatch(push("/status-overview"));
                    break;
                }
                case "meeting": {
                    let meetingReason = MeetingReason.parse(gameState.content);
                    let meetingReasonEnum = MeetingReasonEnum.parse(meetingReason.type);
                    switch (meetingReasonEnum) {
                        case "emergencyMeetingCalled": {
                            const emergencyMeetingCalled = EmergencyMeetingCalled.parse(meetingReason.content);
                            store.dispatch(beginEmergencyMeeting(emergencyMeetingCalled.initiator));
                            store.dispatch(push("/meeting"));
                            break;
                        }
                        case "bodyReported": {
                            const bodyReported = BodyReported.parse(meetingReason.content);
                            store.dispatch(beginReportedBodyMeeting(bodyReported));
                            store.dispatch(push("/meeting"));
                            break;
                        }
                    }
                }
                //                case "reset": {
                //                   store.dispatch(push("/"));
                //                  store.dispatch(deleteAllPlayers());
                //                 break;
                //            }
                default: {
                    throw new Error("Received unknown game state!");
                }

            }
            break;
        }
        case "playerStatus": {
            let playerStatus = PlayerStatus.parse(parsed.content);
            store.dispatch(updatePlayerStatus(playerStatus))
            break;
        }
        case "gameOver": {
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
        case "invalidAction": {
            const invalidAction = InvalidAction.parse(parsed.content);
            store.dispatch(showErrorMessage(invalidAction));
            setTimeout(() => {
                store.dispatch(hideErrorMessage())
            }, 5000)
            break;
        }
        case "successfulKill": {
            var killedPlayer = z.string().parse(parsed.content);
            store.dispatch(setPlayerDead(killedPlayer));
            break;
        }
        case "playerDied": {
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

