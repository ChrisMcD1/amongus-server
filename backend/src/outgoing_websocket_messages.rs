use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::PlayerSerializable;

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
#[rtype(result = "()")]
pub enum OutgoingWebsocketMessage {
    GameState(GameState),
    ResetGame(()),
    ChatMessage(ChatMessage),
    AssignedID(Uuid),
    PlayerStatus(PlayerStatus),
    PlayerRole(SetRole),
    PlayerDied(PlayerDied),
    SuccessfulKill(Uuid),
    InvalidAction(String),
    VotingResults(VotingResults),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum GameState {
    Lobby,
    InGame,
    Meeting(MeetingReason),
    Over(Winner),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "content")]
pub enum MeetingReason {
    BodyReported(BodyReported),
    EmergencyMeetingCalled(EmergencyMeetingCalled),
}

#[derive(Message, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct BodyReported {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct EmergencyMeetingCalled {
    pub initiator: Uuid,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Winner {
    Imposters,
    Crewmates,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub contents: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PlayerConnectionStatus {
    New,
    Disconnected,
    Reconnected,
    Existing,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerStatus {
    pub player: PlayerSerializable,
    pub status: PlayerConnectionStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RoleAssignment {
    Imposter,
    Crewmate,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SetRole {
    pub role: RoleAssignment,
    pub id: Uuid,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerDied {
    pub killer: Uuid,
}

#[derive(Message, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VotingResults {
    pub ejected_player: Option<Uuid>,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ForwardedOutgoingWebsocketMessage {
    pub destination: Uuid,
    pub msg: OutgoingWebsocketMessage,
}
