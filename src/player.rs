use crate::game::Game;
use crate::game_messages::*;
use crate::player_messages::*;
use crate::player_websocket::*;
use actix::dev::*;
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
    pub uuid: Uuid,
}

impl Player {
    pub fn new(name: &str, game: Addr<Game>) -> Self {
        Player {
            role: None,
            name: name.to_string(),
            game,
            websocket: None,
            uuid: Uuid::new_v4(),
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
            uuid: self.uuid,
            name: self.name.clone(),
        });
        ctx.stop();
    }
}

impl Handler<SetRole> for Player {
    type Result = ();
    fn handle(&mut self, msg: SetRole, ctx: &mut Self::Context) -> Self::Result {
        self.role = Some(msg.role);
        self.websocket
            .as_ref()
            .unwrap()
            .do_send(OutboundChatMessage {
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

impl Handler<OutboundChatMessage> for Player {
    type Result = ();
    fn handle(&mut self, msg: OutboundChatMessage, ctx: &mut Self::Context) -> Self::Result {
        if let Some(websocket) = &self.websocket {
            websocket.do_send(msg);
        }
    }
}
