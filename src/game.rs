use crate::incoming_websocket_messages::*;
use crate::internal_messages::*;
use crate::outgoing_websocket_messages::*;
use crate::player::*;
use actix::dev::MessageResponse;
use actix::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg32;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Game {
    state: GameStateEnum,
    players: BTreeMap<Uuid, Addr<Player>>,
    pub rng: Pcg32,
}

impl Game {
    fn send_message_to_all_users(&self, msg: OutgoingWebsocketMessage) {
        self.players
            .iter()
            .for_each(|player| player.1.do_send(msg.clone()))
    }
    pub fn new(seed: u64) -> Self {
        Game {
            state: GameStateEnum::Lobby,
            players: BTreeMap::new(),
            rng: Pcg32::seed_from_u64(seed),
        }
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
        self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            username: msg.name,
            id: msg.id,
            status: PlayerConnectionStatus::Disconnected,
        }));
    }
}

impl Handler<HasGameStarted> for Game {
    type Result = bool;
    fn handle(&mut self, msg: HasGameStarted, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            GameStateEnum::Lobby => false,
            GameStateEnum::InGame => true,
        }
    }
}

impl Handler<RegisterPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayer, ctx: &mut Self::Context) -> Self::Result {
        self.players.insert(msg.id, msg.player);
        self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            username: msg.name,
            id: msg.id,
            status: PlayerConnectionStatus::New,
        }));
    }
}

impl Handler<KillPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: KillPlayer, ctx: &mut Self::Context) -> Self::Result {
        let target = self.players.get(&msg.target).unwrap();
        target.do_send(msg);
    }
}

impl Handler<ReportBody> for Game {
    type Result = ();
    fn handle(&mut self, msg: ReportBody, ctx: &mut Self::Context) -> Self::Result {
        self.players.get(&msg.corpse).unwrap().send(msg);
    }
}

impl Handler<ReportBodyValidated> for Game {
    type Result = ();
    fn handle(&mut self, msg: ReportBodyValidated, ctx: &mut Self::Context) -> Self::Result {
        self.send_message_to_all_users(OutgoingWebsocketMessage::BodyReported(BodyReported {
            corpse: msg.corpse,
            initiator: msg.initiator,
        }));
    }
}

impl Handler<PlayerInvalidAction> for Game {
    type Result = ();
    fn handle(&mut self, msg: PlayerInvalidAction, ctx: &mut Self::Context) -> Self::Result {
        self.players
            .get(&msg.id)
            .unwrap()
            .do_send(OutgoingWebsocketMessage::InvalidAction(msg.error));
    }
}

impl Handler<GetUUID> for Game {
    type Result = Arc<Uuid>;
    fn handle(&mut self, msg: GetUUID, ctx: &mut Self::Context) -> Self::Result {
        Arc::new(Uuid::from_bytes(self.rng.gen())).clone()
    }
}

impl Handler<ForwardedOutgoingWebsocketMessage> for Game {
    type Result = ();
    fn handle(
        &mut self,
        msg: ForwardedOutgoingWebsocketMessage,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        self.players.get(&msg.destination).unwrap().do_send(msg.msg);
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        self.state = GameStateEnum::InGame;
        let player_count = self.players.len();
        let mut imposter_count = get_imposter_count(player_count);
        let mut imposters: HashSet<Uuid> = HashSet::new();
        let mut player_roles: BTreeMap<Uuid, RoleAssignment> = self
            .players
            .clone()
            .iter()
            .map(|player| (player.0.clone(), RoleAssignment::Crewmate))
            .collect();
        while imposter_count > 0 {
            let imposter_index = self.rng.gen_range(0..player_count);
            let player = self.players.iter().nth(imposter_index).unwrap();
            if imposters.contains(player.0) {
                continue;
            }
            imposters.insert(player.0.clone());
            player_roles.insert(player.0.clone(), RoleAssignment::Imposter);
            imposter_count = imposter_count - 1;
        }
        player_roles.into_iter().for_each(|role| {
            self.players
                .get(&role.0)
                .unwrap()
                .do_send(SetRole { role: role.1 });
        });

        self.send_message_to_all_users(OutgoingWebsocketMessage::GameState(GameState {
            state: GameStateEnum::InGame,
        }));
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
