use crate::game::Game;
use crate::incoming_websocket_messages::*;
use crate::internal_messages::*;
use crate::outgoing_websocket_messages::*;
use actix::dev::*;
use actix_web_actors::ws;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct PlayerWithWebsocket {
    pub role: Option<Role>,
    pub name: String,
    pub alive: bool,
    pub id: Uuid,
    pub websocket: Option<Addr<PlayerWebsocket>>,
}

impl PlayerWithWebsocket {
    pub fn new(name: &str, id: Uuid) -> Self {
        PlayerWithWebsocket {
            role: None,
            name: name.to_string(),
            alive: true,
            id,
            websocket: None,
        }
    }
    pub fn close_websocket(&self) {
        match &self.websocket {
            Some(websocket) => websocket.do_send(CloseWebsocket {}),
            None => {
                println!(
                    "Trying to close websocket for {:?} that was never opened",
                    self.id
                )
            }
        }
    }
    pub fn set_websocket_address(&mut self, websocket: Addr<PlayerWebsocket>) {
        self.websocket = Some(websocket);
    }
    pub fn send_outgoing_message(&self, msg: OutgoingWebsocketMessage) {
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
        self.send_outgoing_message(OutgoingWebsocketMessage::PlayerRole(SetRole { role }));
    }
}

impl Handler<CloseWebsocket> for PlayerWebsocket {
    type Result = ();
    fn handle(&mut self, msg: CloseWebsocket, ctx: &mut Self::Context) -> Self::Result {
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
            Ok(ws::Message::Text(s)) => self.handle_incoming_message(s.to_string(), ctx),

            Err(e) => panic!("{}", e),
        }
    }
}

impl Actor for PlayerWebsocket {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.game.do_send(RegisterPlayerWebsocket {
            id: self.id,
            websocket: ctx.address(),
        });
    }
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Stopping websocket");
        self.game
            .do_send(PlayerWithWebsocketDisconnected { id: self.id });
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
    fn handle_incoming_message(&mut self, msg: String, ctx: &mut ws::WebsocketContext<Self>) {
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

#[derive(Debug)]
pub struct Player {
    pub role: Option<Role>,
    pub name: String,
    alive: bool,
    game: Addr<Game>,
    heartbeat: Instant,
    pub id: Uuid,
}

impl Player {
    pub fn new(name: &str, game: Addr<Game>, id: Uuid) -> Self {
        Player {
            role: None,
            name: name.to_string(),
            alive: true,
            game,
            heartbeat: Instant::now(),
            id,
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
    fn handle_valid_incoming_message(
        &mut self,
        msg: IncomingWebsocketMessage,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        match msg {
            IncomingWebsocketMessage::KillPlayer(kill) => match self.role.unwrap() {
                Role::Imposter(ref mut imposter) => {
                    if !imposter.kill_is_off_cooldown() {
                        ctx.notify(OutgoingWebsocketMessage::InvalidAction(format!(
                            "You are not off kill cooldown yet. Try again in {:#?}",
                            imposter.cooldown_remaining()
                        )));
                        return;
                    }
                    self.role = Some(Role::Imposter(imposter.reset_kill_cooldown()));
                    self.game.do_send(InternalKillPlayer {
                        target: kill.target,
                        initiator: self.id,
                    });
                }
                _ => {
                    ctx.notify(OutgoingWebsocketMessage::InvalidAction(
                        "Good try, but you can only kill people if you are an imposter!"
                            .to_string(),
                    ));
                }
            },
            IncomingWebsocketMessage::ReportBody(report) => {
                self.game.do_send(InternalReportBody {
                    corpse: report.corpse,
                    initiator: self.id,
                });
            }
            IncomingWebsocketMessage::Vote(vote) => {
                self.game.do_send(InternalVote {
                    target: vote.target,
                    initiator: self.id,
                });
            }
        }
    }
    fn handle_incoming_message(&mut self, msg: String, ctx: &mut ws::WebsocketContext<Self>) {
        let msg = serde_json::from_str::<IncomingWebsocketMessage>(&msg);
        match msg {
            Ok(msg) => self.handle_valid_incoming_message(msg, ctx),
            Err(err) => ctx.notify(OutgoingWebsocketMessage::InvalidAction(err.to_string())),
        }
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
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Stopping websocket");
        self.game.do_send(PlayerDisconnected {
            id: self.id,
            name: self.name.clone(),
        });
        Running::Stop
    }
}

impl Handler<InternalSetPlayerRole> for Player {
    type Result = ();
    fn handle(&mut self, msg: InternalSetPlayerRole, ctx: &mut Self::Context) -> Self::Result {
        self.role = Some(match msg.role {
            RoleAssignment::Crewmate => Role::Crewmate,
            RoleAssignment::Imposter => Role::Imposter(Imposter::new(msg.kill_cooldown)),
        });
        ctx.notify(OutgoingWebsocketMessage::PlayerRole(SetRole {
            role: msg.role,
        }));
    }
}

impl Handler<SetPlayerAlive> for Player {
    type Result = ();
    fn handle(&mut self, msg: SetPlayerAlive, _ctx: &mut Self::Context) -> Self::Result {
        self.alive = msg.alive;
    }
}

impl Handler<OutgoingWebsocketMessage> for Player {
    type Result = ();
    fn handle(&mut self, msg: OutgoingWebsocketMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg_serialized = serde_json::to_string(&msg).unwrap();
        println!("Sending to {:?} msg: {:?}", self.name, msg_serialized);
        ctx.text(msg_serialized);
    }
}

impl Handler<InternalReportBody> for Player {
    type Result = ();
    fn handle(&mut self, msg: InternalReportBody, _ctx: &mut Self::Context) -> Self::Result {
        if self.alive {
            self.game.do_send(PlayerInvalidAction {
                id: msg.initiator,
                error: format!(
                    "You cannot report {}'s body since they are alive",
                    self.name
                ),
            });
            return;
        }
        self.game.do_send(ReportBodyValidated {
            corpse: msg.corpse,
            initiator: msg.initiator,
        });
    }
}

impl Handler<InternalKillPlayer> for Player {
    type Result = ();
    fn handle(&mut self, msg: InternalKillPlayer, ctx: &mut Self::Context) -> Self::Result {
        match self.role.unwrap() {
            Role::Crewmate => {
                if !self.alive {
                    self.game.do_send(PlayerInvalidAction {
                        id: msg.initiator,
                        error: format!("You cannot kill {} since they are already dead", self.name),
                    });
                    return;
                }
                self.game.do_send(ForwardedOutgoingWebsocketMessage {
                    destination: msg.initiator,
                    msg: OutgoingWebsocketMessage::SuccessfulKill(),
                });
                self.alive = false;
                ctx.notify(OutgoingWebsocketMessage::PlayerDied(PlayerDied {
                    killer: msg.initiator,
                }));
            }
            Role::Imposter(_) => self.game.do_send(PlayerInvalidAction {
                id: msg.initiator,
                error: "You cannot kill a fellow imposter, silly".to_string(),
            }),
        }
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
                self.handle_incoming_message(s.to_string(), ctx);
            }

            Err(e) => panic!("{}", e),
        }
    }
}
