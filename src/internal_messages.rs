use crate::player::Player;
use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub id: Uuid,
    pub name: String,
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
pub struct PlayerInvalidAction {
    pub id: Uuid,
    pub error: String,
}
