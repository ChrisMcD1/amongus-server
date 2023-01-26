use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[rtype(result = "()")]
pub enum OutgoingWebsocketMessage {
    ChatMessage(ChatMessage),
    PlayerStatus(PlayerStatus),
    GameState(GameState),
    PlayerRole(SetRole),
    PlayerDied(PlayerDied),
    SuccessfulKill(),
    InvalidAction(String),
    BodyReported(BodyReported),
    VotingResults(VotingResults),
}

#[derive(Message, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VotingResults {
    pub ejected_player: Option<Uuid>,
}

#[derive(Message, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct BodyReported {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ForwardedOutgoingWebsocketMessage {
    pub destination: Uuid,
    pub msg: OutgoingWebsocketMessage,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerDied {
    pub killer: Uuid,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub contents: String,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerStatus {
    pub username: String,
    pub id: Uuid,
    pub status: PlayerConnectionStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlayerConnectionStatus {
    New,
    Disconnected,
    Reconnected,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct GameState {
    pub state: GameStateEnum,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameStateEnum {
    Lobby,
    InGame,
}

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct SetRole {
    pub role: RoleAssignment,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RoleAssignment {
    Imposter,
    Crewmate,
}
