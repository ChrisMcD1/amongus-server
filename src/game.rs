use crate::game_messages::*;
use crate::player::*;
use crate::player_messages::*;
use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
enum GameState {
    Lobby,
    InGame,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    players: HashMap<Uuid, Addr<Player>>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            state: GameState::Lobby,
            players: HashMap::new(),
        }
    }
}

impl Game {
    fn send_message_to_all_players(&self, msg: &str) {
        self.players.iter().for_each(|player| {
            player.1.do_send(OutboundChatMessage {
                contents: msg.to_string(),
            })
        })
    }
}

impl Actor for Game {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PrintGameState {}

impl Handler<PrintGameState> for Game {
    type Result = ();
    fn handle(&mut self, msg: PrintGameState, ctx: &mut Self::Context) -> Self::Result {
        println!("{:#?}", self);
    }
}

impl Handler<PlayerDisconnected> for Game {
    type Result = ();
    fn handle(&mut self, msg: PlayerDisconnected, ctx: &mut Self::Context) -> Self::Result {
        self.players.remove(&msg.uuid);
        self.send_message_to_all_players(&format!("{} has disconnected.", msg.name));
    }
}

impl Handler<HasGameStarted> for Game {
    type Result = bool;
    fn handle(&mut self, msg: HasGameStarted, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            GameState::Lobby => false,
            GameState::InGame => true,
        }
    }
}

impl Handler<RegisterPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayer, ctx: &mut Self::Context) -> Self::Result {
        self.players.insert(msg.uuid, msg.player);
        self.send_message_to_all_players(&format!("{} has connected.", msg.name));
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        self.state = GameState::InGame;
        self.send_message_to_all_players("Game has begun!");
    }
}
