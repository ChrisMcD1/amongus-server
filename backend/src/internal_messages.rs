use crate::incoming_websocket_messages::IncomingWebsocketMessage;
use crate::player::PlayerWebsocket;
use actix::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct IncomingMessageInternal {
    pub initiator: Uuid,
    pub incoming: IncomingWebsocketMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayer {
    pub id: Uuid,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseWebsocket {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayerWebsocket {
    pub id: Uuid,
    pub websocket: Addr<PlayerWebsocket>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartGame {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ResetGame {}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct HasGameStarted {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EndVoting {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartMeeting {}

#[derive(Message)]
#[rtype(result = "Arc<Uuid>")]
pub struct GetNextUUID {}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct PlayerExists {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct GetPlayerColor {
    pub id: Uuid,
}
