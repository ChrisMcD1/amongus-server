use std::time::{Duration, Instant};

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
    SuccessfulKill(),
    InvalidAction(String),
}

#[derive(Message, Debug, Serialize, Clone)]
#[rtype(result = "()")]
pub struct ForwardedOutgoingWebsocketMessage {
    pub destination: Uuid,
    pub msg: OutgoingWebsocketMessage,
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

const KILL_COOLDOWN: Duration = Duration::from_secs(60);

impl Imposter {
    pub fn new() -> Self {
        Imposter {
            last_kill_time: Instant::now(),
        }
    }
    pub fn kill_is_off_cooldown(&self) -> bool {
        self.last_kill_time.elapsed() > KILL_COOLDOWN
    }
    pub fn cooldown_remaining(&self) -> Duration {
        KILL_COOLDOWN - self.last_kill_time.elapsed()
    }
    pub fn reset_kill_cooldown(&mut self) {
        self.last_kill_time = Instant::now();
    }
}
