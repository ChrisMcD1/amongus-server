use std::time::Instant;

use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub enum OutgoingWebsocketMessage {
    ChatMessage(ChatMessage),
    PlayerStatus(PlayerStatus),
    GameState(GameState),
    PlayerRole(SetRole),
    PlayerDied(PlayerDied),
    InvalidAction(String),
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub struct PlayerDied {}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub contents: String,
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub struct PlayerStatus {
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
pub struct GameState {
    pub state: GameStateEnum,
}

#[derive(Debug, Serialize, Clone)]
pub enum GameStateEnum {
    Lobby,
    InGame,
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub struct SetRole {
    pub role: Role,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Role {
    Imposter(Imposter),
    Crewmate,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Imposter {
    #[serde(with = "approx_instant")]
    last_kill_time: Instant,
}

mod approx_instant {
    use serde::{Serialize, Serializer};
    use std::time::Instant;

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.elapsed();
        duration.serialize(serializer)
    }
}

impl Imposter {
    pub fn new() -> Self {
        Imposter {
            last_kill_time: Instant::now(),
        }
    }
}
