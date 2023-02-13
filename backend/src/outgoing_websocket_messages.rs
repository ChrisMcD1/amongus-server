use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "content")]
#[rtype(result = "()")]
pub enum OutgoingWebsocketMessage {
    ChatMessage(ChatMessage),
    PlayerStatus(PlayerStatus),
    GameState(GameState),
    PlayerRole(SetRole),
    PlayerDied(PlayerDied),
    SuccessfulKill(()),
    InvalidAction(String),
    BodyReported(BodyReported),
    VotingResults(VotingResults),
    GameOver(Winner),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Winner {
    Imposters,
    Crewmates,
}

#[derive(Message, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct VotingResults {
    pub ejected_player: Option<Uuid>,
}

#[derive(Message, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct BodyReported {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ForwardedOutgoingWebsocketMessage {
    pub destination: Uuid,
    pub msg: OutgoingWebsocketMessage,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerDied {
    pub killer: Uuid,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub contents: String,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct PlayerStatus {
    pub username: String,
    pub id: Uuid,
    pub color: String,
    pub status: PlayerConnectionStatus,
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
pub struct GameState {
    pub state: GameStateEnum,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GameStateEnum {
    Lobby,
    InGame,
}

#[derive(Message, PartialEq, Debug, Serialize, Deserialize, Clone)]
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
