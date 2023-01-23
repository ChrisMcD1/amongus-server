use crate::game_messages::*;
use crate::player_messages::*;
use crate::player_websocket_messages::*;
use crate::Game;
use actix::dev::*;
use actix_web_actors::ws;
use serde::Serialize;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Imposter,
    Crewmate,
}

#[derive(Debug)]
pub struct Player {
    pub role: Option<Role>,
    pub name: String,
    game: Addr<Game>,
    heartbeat: Instant,
    pub id: Uuid,
}

impl Player {
    pub fn new(name: &str, game: Addr<Game>) -> Self {
        Player {
            role: None,
            name: name.to_string(),
            game,
            heartbeat: Instant::now(),
            id: Uuid::new_v4(),
        }
    }
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |connection, ctx| {
            if Instant::now().duration_since(connection.heartbeat) > CLIENT_TIMEOUT {
                println!("Disconnecting from a faild hearatbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
    fn handle_message(&self, msg: String) {
        println!("Got message: {msg}");
    }
}

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.game.do_send(RegisterPlayer {
            id: self.id,
            name: self.name.clone(),
            player: ctx.address(),
        });
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        println!("Stopping websocket");
        Running::Stop
    }
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
        ctx.text(format!("You have been assigned a role of {:#?}", msg.role));
    }
}

impl<T> Handler<T> for Player
where
    T: Message<Result = ()> + Serialize + Debug + Send + 'static,
{
    type Result = ();
    fn handle(&mut self, msg: T, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg)
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            // Not equiped to handle big messages
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => {}
            Ok(ws::Message::Text(s)) => {
                self.handle_message(s.to_string());
            }

            Err(e) => panic!("{}", e),
        }
    }
}
