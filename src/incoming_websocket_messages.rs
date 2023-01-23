use actix::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
#[derive(Message, Debug, Deserialize)]
#[rtype(result = "()")]
pub enum IncomingWebsocketMessage {
    KillPlayer(KillPlayer),
}

#[derive(Message, Debug, Deserialize)]
#[rtype(result = "()")]
pub struct KillPlayer {
    pub target: Uuid,
    pub imposter: Uuid,
}
