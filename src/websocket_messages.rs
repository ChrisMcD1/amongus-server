use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Message, Debug, Serialize)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub struct ChatMessage {
    pub contents: String,
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub struct PlayerStatusMessage {
    pub username: String,
    pub id: Uuid,
    pub status: PlayerStatus,
}

#[derive(Debug, Serialize, Clone)]
pub enum PlayerStatus {
    New,
    Disconnected,
    Reconnected,
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub struct GameStateMessage {
    pub state: GameState,
}

#[derive(Debug, Serialize, Clone)]
pub enum GameState {
    Lobby,
    InGame,
}

#[derive(Message, Debug, Serialize)]
#[rtype(result = "()")]
pub struct SetRole {
    pub role: Role,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Role {
    Imposter,
    Crewmate,
}
