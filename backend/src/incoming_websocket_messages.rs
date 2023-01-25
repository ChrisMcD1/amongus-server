use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub enum IncomingWebsocketMessage {
    KillPlayer(KillPlayer),
    ReportBody(ReportBody),
    Vote(Vote),
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct ReportBody {
    pub corpse: Uuid,
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct KillPlayer {
    pub target: Uuid,
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct Vote {
    pub target: Uuid,
}
