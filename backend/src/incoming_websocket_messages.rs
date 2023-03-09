use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "content")]
#[rtype(result = "()")]
pub enum IncomingWebsocketMessage {
    KillPlayer(KillPlayer),
    ReportBody(ReportBody),
    Vote(Vote),
    ChooseColor(ChooseColor),
    CallEmergencyMeeting,
}

#[derive(Message, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ReportBody {
    pub corpse: Uuid,
}

#[derive(Message, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct KillPlayer {
    pub target: Uuid,
}

#[derive(Message, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct Vote {
    pub target: Uuid,
}

#[derive(Message, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ChooseColor {
    pub color: String,
}
