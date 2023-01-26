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
pub struct PlayerWithWebsocketDisconnected {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayerWithWebsocket {
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

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct InternalVote {
    pub target: Uuid,
    pub initiator: Uuid,
}
