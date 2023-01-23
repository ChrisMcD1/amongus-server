use crate::game_messages::*;
use crate::player_messages::*;
use crate::player_websocket::*;
use crate::player_websocket_messages::*;
use crate::Game;
use actix::dev::*;
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Imposter,
    Crewmate,
}

#[derive(Debug)]
pub struct Player {
    pub role: Option<Role>,
    pub name: String,
    websocket: Option<Addr<PlayerWebsocket>>,
    game: Addr<Game>,
    pub id: Uuid,
}

impl Player {
    pub fn new(name: &str, game: Addr<Game>) -> Self {
        Player {
            role: None,
            name: name.to_string(),
            game,
            websocket: None,
            id: Uuid::new_v4(),
        }
    }
}

impl Actor for Player {
    type Context = Context<Self>;
}

impl Handler<Disconnected> for Player {
    type Result = ();
    fn handle(&mut self, msg: Disconnected, ctx: &mut Self::Context) -> Self::Result {
        self.game.do_send(PlayerDisconnected {
            id: self.id,
            name: self.name.clone(),
        });
        ctx.stop();
    }
}

impl Handler<SetRole> for Player {
    type Result = ();
    fn handle(&mut self, msg: SetRole, ctx: &mut Self::Context) -> Self::Result {
        self.role = Some(msg.role);
        self.websocket.as_ref().unwrap().do_send(ChatMessage {
            contents: format!("You have been assigned a role of {:#?}", msg.role),
        })
    }
}

impl Handler<RegisterWebSocket> for Player {
    type Result = ();
    fn handle(&mut self, msg: RegisterWebSocket, ctx: &mut Self::Context) -> Self::Result {
        self.websocket = Some(msg.socket);
    }
}

impl<T> Handler<T> for Player
where
    T: Message<Result = ()> + Serialize + Debug + Send + 'static,
{
    type Result = ();
    fn handle(&mut self, msg: T, ctx: &mut Self::Context) -> Self::Result {
        if let Some(websocket) = &self.websocket {
            websocket.do_send(msg);
        }
    }
}
