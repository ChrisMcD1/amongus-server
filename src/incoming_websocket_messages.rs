use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub enum IncomingWebsocketMessage {
    KillPlayer(KillPlayer),
    ReportBody(ReportBody),
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct ReportBody {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct KillPlayer {
    pub target: Uuid,
    pub initiator: Uuid,
}
