use crate::player_websocket::PlayerWebsocket;
use crate::Role;
use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterWebSocket {
    pub socket: Addr<PlayerWebsocket>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetRole {
    pub role: Role,
}
