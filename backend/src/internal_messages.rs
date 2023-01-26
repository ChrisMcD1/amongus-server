use crate::incoming_websocket_messages::IncomingWebsocketMessage;
use crate::outgoing_websocket_messages::RoleAssignment;
use crate::player::{Player, PlayerWebsocket};
use actix::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub id: Uuid,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct IncomingMessageInternal {
    pub initiator: Uuid,
    pub incoming: IncomingWebsocketMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerWithWebsocketDisconnected {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayer {
    pub player: Addr<Player>,
    pub id: Uuid,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayerWithWebsocket {
    pub id: Uuid,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseWebsocket {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayerWebsocket {
    pub id: Uuid,
    pub websocket: Addr<PlayerWebsocket>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReportBodyValidated {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartGame {}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct HasGameStarted {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnected {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EndVoting {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartMeeting {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerInvalidAction {
    pub id: Uuid,
    pub error: String,
}

#[derive(Message)]
#[rtype(result = "Arc<Uuid>")]
pub struct GetUUID {}

#[derive(Message)]
#[rtype(result = "Arc<Duration>")]
pub struct GetKillCooldown {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InternalSetPlayerRole {
    pub role: RoleAssignment,
    pub kill_cooldown: Duration,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetPlayerAlive {
    pub alive: bool,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct InternalKillPlayer {
    pub target: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct InternalReportBody {
    pub corpse: Uuid,
    pub initiator: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct InternalVote {
    pub target: Uuid,
    pub initiator: Uuid,
}
