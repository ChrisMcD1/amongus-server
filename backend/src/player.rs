use crate::game::Game;
use crate::incoming_websocket_messages::*;
use crate::internal_messages::*;
use crate::outgoing_websocket_messages::*;
use actix::dev::*;
use actix_web_actors::ws;
use bytestring::ByteString;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(skip_serializing, skip_deserializing)]
    pub role: Option<Role>,
    pub username: String,
    pub alive: bool,
    pub color: String,
    pub has_connected_previously: bool,
    pub id: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    pub websocket: Option<Addr<PlayerWebsocket>>,
}

impl Player {
    pub fn new(name: &str, id: Uuid) -> Self {
        Player {
            role: None,
            username: name.to_string(),
            alive: true,
            color: "#FFFFFF".to_string(),
            has_connected_previously: false,
            id,
            websocket: None,
        }
    }
    pub fn close_websocket(&mut self) {
        match &self.websocket {
            Some(websocket) => websocket.do_send(CloseWebsocket {}),
            None => {
                println!(
                    "Trying to close websocket for {:?} that was never opened",
                    self.id
                )
            }
        }
        self.websocket = None;
    }
    pub fn set_websocket_address(&mut self, websocket: Addr<PlayerWebsocket>) {
        self.websocket = Some(websocket);
        self.has_connected_previously = true;
    }
    pub fn send_outgoing_message(&mut self, msg: OutgoingWebsocketMessage) {
        self.send_websocket_message_internal(msg);
    }
    fn send_websocket_message_internal(&self, msg: OutgoingWebsocketMessage) {
        match &self.websocket {
            Some(websocket) => websocket.do_send(msg),
            None => {
                println!(
                    "Trying to send message to websocket for {:?} that is closed",
                    self.id
                )
            }
        }
    }
    pub fn set_role(&mut self, role: RoleAssignment, kill_cooldown: Duration) {
        self.role = Some(match role {
            RoleAssignment::Crewmate => Role::Crewmate,
            RoleAssignment::Imposter => Role::Imposter(Imposter::new(kill_cooldown)),
        });
        self.send_outgoing_message(OutgoingWebsocketMessage::PlayerRole(SetRole {
            role,
            id: self.id,
        }));
    }
    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }
}

impl Handler<CloseWebsocket> for PlayerWebsocket {
    type Result = ();
    fn handle(&mut self, _msg: CloseWebsocket, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

pub struct PlayerWebsocket {
    id: Uuid,
    heartbeat: Instant,
    game: Addr<Game>,
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
            Ok(ws::Message::Text(s)) => self.handle_incoming_message(s, ctx),

            Err(e) => panic!("{}", e),
        }
    }
}

impl Actor for PlayerWebsocket {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        ctx.address()
            .do_send(OutgoingWebsocketMessage::AssignedID(self.id));
        self.game.do_send(RegisterPlayerWebsocket {
            id: self.id,
            websocket: ctx.address(),
        });
    }
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Stopping websocket");
        self.game.do_send(PlayerDisconnected { id: self.id });
        Running::Stop
    }
}

impl PlayerWebsocket {
    pub fn new(id: Uuid, game: Addr<Game>) -> Self {
        PlayerWebsocket {
            id,
            heartbeat: Instant::now(),
            game,
        }
    }
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |connection, ctx| {
            if Instant::now().duration_since(connection.heartbeat) > CLIENT_TIMEOUT {
                println!("Disconnecting from a failed hearatbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
    fn handle_incoming_message(&mut self, msg: ByteString, ctx: &mut ws::WebsocketContext<Self>) {
        println!(
            "Received a message from the user {:?} of {:?}",
            self.id, msg
        );
        let msg = serde_json::from_str::<IncomingWebsocketMessage>(&msg);
        match msg {
            Ok(msg) => self.game.do_send(IncomingMessageInternal {
                initiator: self.id,
                incoming: msg,
            }),
            Err(err) => ctx.notify(OutgoingWebsocketMessage::InvalidAction(err.to_string())),
        }
    }
}

impl Handler<OutgoingWebsocketMessage> for PlayerWebsocket {
    type Result = ();
    fn handle(&mut self, msg: OutgoingWebsocketMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg_serialized = serde_json::to_string(&msg).unwrap();
        println!("Sending to {:?} msg: {:?}", self.id, msg_serialized);
        ctx.text(msg_serialized);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Imposter(Imposter),
    Crewmate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Imposter {
    last_kill_time: Instant,
    kill_cooldown: Duration,
}

impl Imposter {
    pub fn new(kill_cooldown: Duration) -> Self {
        Imposter {
            last_kill_time: Instant::now(),
            kill_cooldown,
        }
    }
    pub fn kill_is_off_cooldown(&self) -> bool {
        self.last_kill_time.elapsed() > self.kill_cooldown
    }
    pub fn cooldown_remaining(&self) -> Duration {
        self.kill_cooldown - self.last_kill_time.elapsed()
    }
    pub fn reset_kill_cooldown(&self) -> Self {
        Imposter {
            last_kill_time: Instant::now(),
            kill_cooldown: self.kill_cooldown,
        }
    }
}
