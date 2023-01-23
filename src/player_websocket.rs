use crate::player::Player;
use crate::player_messages::*;
use actix::dev::*;
use actix::ActorContext;
use actix::AsyncContext;
use actix::{Actor, Addr, Running, StreamHandler};
use actix_web_actors::ws;
use serde::Serialize;
use std::fmt::Debug;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// The goal of this struct is to handle all of the serialization and deserialization
// of the websocket messages, so that the Player struct can focus entirely on business logic
// This struct should only know how to serialize and deserialize, and should not know what the
// messages actually mean.
#[derive(Debug)]
pub struct PlayerWebsocket {
    player: Addr<Player>,
    heartbeat: Instant,
}

impl Actor for PlayerWebsocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.player.do_send(RegisterWebSocket {
            socket: ctx.address(),
        });
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.player.do_send(Disconnected {});
        println!("Stopping websocket");
        Running::Stop
    }
}

impl PlayerWebsocket {
    pub fn new(player: Addr<Player>) -> Self {
        PlayerWebsocket {
            player,
            heartbeat: Instant::now(),
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

impl<T> Handler<T> for PlayerWebsocket
where
    T: Message<Result = ()> + Serialize + Debug,
{
    type Result = ();
    fn handle(&mut self, msg: T, ctx: &mut Self::Context) -> Self::Result {
        println!("Should be sending outbound message of {:?}", msg);
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl<A, M> MessageResponse<A, M> for PlayerWebsocket
where
    A: Actor,
    M: Message<Result = PlayerWebsocket>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self).unwrap();
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWebsocket {
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
