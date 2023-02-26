import { updatePlayerStatus } from './state/playersSlice';
import store from './state/store';
import { ChatMessage, PlayerStatus, PreZodMessage } from "./Messages/fromServer";
import z from "zod";


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
