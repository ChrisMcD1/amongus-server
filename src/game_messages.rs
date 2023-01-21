use crate::player::Player;
use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PlayerDisconnected {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterPlayer {
    pub player: Addr<Player>,
    pub uuid: Uuid,
    pub name: String,
}
