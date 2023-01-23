use crate::game_messages::*;
use crate::player::*;
use crate::player_messages::*;
use crate::player_websocket_messages::*;
use actix::prelude::*;
use rand::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub enum GameState {
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
    fn send_chat_message_to_all_players(&self, msg: &str) {
        self.players.iter().for_each(|player| {
            player.1.do_send(ChatMessage {
                contents: msg.to_string(),
            })
        })
    }
}

impl Game {
    fn send_message_to_users<T>(&self, msg: T)
    where
        T: Message<Result = ()> + Serialize + Debug + Clone + Send + 'static,
    {
        self.players
            .iter()
            .for_each(|player| player.1.do_send(msg.clone()))
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
        self.players.remove(&msg.id);
        self.send_message_to_users(PlayerStatusMessage {
            username: msg.name,
            id: msg.id,
            status: PlayerStatus::Disconnected,
        });
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
        self.players.insert(msg.id, msg.player);
        self.send_message_to_users(PlayerStatusMessage {
            username: msg.name,
            id: msg.id,
            status: PlayerStatus::New,
        });
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        self.state = GameState::InGame;
        let player_count = self.players.len();
        let mut imposter_count = get_imposter_count(player_count);
        let mut imposters: HashSet<Uuid> = HashSet::new();
        let mut player_roles: HashMap<Uuid, Role> = self
            .players
            .clone()
            .iter()
            .map(|player| (player.0.clone(), Role::Crewmate))
            .collect();
        let mut rng = rand::thread_rng();
        while imposter_count > 0 {
            let imposter_index = rng.gen_range(0..player_count);
            let player = self.players.iter().nth(imposter_index).unwrap();
            if imposters.contains(player.0) {
                continue;
            }
            imposters.insert(player.0.clone());
            player_roles.insert(player.0.clone(), Role::Imposter);
            imposter_count = imposter_count - 1;
        }
        player_roles.into_iter().for_each(|role| {
            self.players
                .get(&role.0)
                .unwrap()
                .do_send(SetRole { role: role.1 });
        });

        self.send_message_to_users(GameStateMessage {
            state: GameState::InGame,
        });
    }
}

fn get_imposter_count(player_count: usize) -> usize {
    if player_count <= 4 {
        return 1;
    } else if player_count <= 7 {
        return 2;
    } else {
        return 3;
    }
}
