use actix::prelude::*;
use serde::{Deserialize, Serialize};
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
    pub status: PlayerConnectionStatus,
}

#[derive(Debug, Serialize, Clone)]
pub enum PlayerConnectionStatus {
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

#[derive(Message, Debug, Deserialize)]
#[rtype(result = "()")]
pub struct KillPlayer {
    pub target: Uuid,
    pub imposter: Uuid,
}
