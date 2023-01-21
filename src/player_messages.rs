use crate::player_websocket::PlayerWebsocket;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OutboundChatMessage {
    pub contents: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterWebSocket {
    pub socket: Addr<PlayerWebsocket>,
}
