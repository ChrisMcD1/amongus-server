use crate::internal_messages::*;
use crate::outgoing_websocket_messages::*;
use crate::player::*;
use actix::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg32;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug)]
pub struct Game {
    state: GameStateEnum,
    alive_player_count: u32,
    players: BTreeMap<Uuid, Addr<Player>>,
    kill_cooldown: Duration,
    pub rng: Pcg32,
    meeting: Option<Meeting>,
}

#[derive(Debug)]
pub struct Meeting {
    alive_player_count: u32,
    votes: BTreeMap<Uuid, Uuid>,
}

impl Meeting {
    pub fn new(alive_player_count: u32) -> Self {
        Meeting {
            alive_player_count,
            votes: BTreeMap::new(),
        }
    }
    pub fn add_vote(&mut self, vote_by: Uuid, vote_for: Uuid) {
        self.votes.insert(vote_by, vote_for);
    }
    pub fn all_players_voted(&self) -> bool {
        u32::try_from(self.votes.len()).expect("Shouldn't exceed a u32 number of players lol")
            == self.alive_player_count
    }
    pub fn person_voted_out(&self) -> Option<Uuid> {
        let vote_threshold = (f64::from(self.alive_player_count) / 2f64).ceil() as u32;
        let mut votes_for_each: BTreeMap<Uuid, u32> = BTreeMap::new();
        for vote in self.votes.iter() {
            let vote_for = vote.1;
            match votes_for_each.get(vote_for) {
                Some(votes) => votes_for_each.insert(*vote_for, votes + 1),
                None => votes_for_each.insert(*vote_for, 1),
            };
        }
        let highest_person_votes = votes_for_each.iter().reduce(|accum, item| {
            if item.1 > accum.1 {
                return item;
            } else {
                accum
            }
        })?;
        if *highest_person_votes.1 >= vote_threshold {
            Some(*highest_person_votes.0)
        } else {
            None
        }
    }
}

const VOTING_TIME: Duration = Duration::from_secs(60);

impl Game {
    fn send_message_to_all_users(&self, msg: OutgoingWebsocketMessage) {
        self.players
            .iter()
            .for_each(|player| player.1.do_send(msg.clone()))
    }
    pub fn new(kill_cooldown: Duration, seed: u64) -> Self {
        Game {
            state: GameStateEnum::Lobby,
            players: BTreeMap::new(),
            kill_cooldown,
            rng: Pcg32::seed_from_u64(seed),
            meeting: None,
            alive_player_count: 0,
        }
    }
    pub fn start_meeting(&mut self, ctx: &mut Context<Self>) {
        self.meeting = Some(Meeting::new(self.alive_player_count));
        println!("Started meeting as {:?}", self.meeting);
        ctx.notify_later(EndVoting {}, VOTING_TIME);
    }
    pub fn end_meeting(&mut self) {
        println!("Stopping meeting");
        match self.meeting.as_ref() {
            Some(meeting) => {
                let voted_out_user = meeting.person_voted_out();
                self.meeting = None;
                self.send_message_to_all_users(OutgoingWebsocketMessage::VotingResults(
                    VotingResults {
                        ejected_player: voted_out_user,
                    },
                ))
            }
            None => {
                println!("Received Message to end meeting, but it has already ended!")
            }
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
    fn handle(&mut self, _msg: PrintGameState, _ctx: &mut Self::Context) -> Self::Result {
        println!("{:#?}", self);
    }
}

impl Handler<EndVoting> for Game {
    type Result = ();
    fn handle(&mut self, _msg: EndVoting, _ctx: &mut Self::Context) -> Self::Result {
        self.end_meeting();
    }
}

impl Handler<StartMeeting> for Game {
    type Result = ();
    fn handle(&mut self, _msg: StartMeeting, ctx: &mut Self::Context) -> Self::Result {
        self.start_meeting(ctx);
    }
}

impl Handler<InternalVote> for Game {
    type Result = ();
    fn handle(&mut self, msg: InternalVote, ctx: &mut Self::Context) -> Self::Result {
        match self.meeting.as_mut() {
            Some(meeting) => {
                meeting.add_vote(msg.initiator, msg.target);
                if meeting.all_players_voted() {
                    ctx.notify(EndVoting {});
                }
            }
            None => println!("Cannot vote without a meeting active!"),
        }
    }
}

impl Handler<PlayerDisconnected> for Game {
    type Result = ();
    fn handle(&mut self, msg: PlayerDisconnected, _ctx: &mut Self::Context) -> Self::Result {
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
    fn handle(&mut self, _msg: HasGameStarted, _ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            GameStateEnum::Lobby => false,
            GameStateEnum::InGame => true,
        }
    }
}

impl Handler<RegisterPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayer, _ctx: &mut Self::Context) -> Self::Result {
        self.players.insert(msg.id, msg.player);
        self.alive_player_count = self.alive_player_count + 1;
        self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            username: msg.name,
            id: msg.id,
            status: PlayerConnectionStatus::New,
        }));
    }
}

impl Handler<InternalKillPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: InternalKillPlayer, _ctx: &mut Self::Context) -> Self::Result {
        let target = self.players.get(&msg.target).unwrap();
        target.do_send(msg);
    }
}

impl Handler<InternalReportBody> for Game {
    type Result = ();
    fn handle(&mut self, msg: InternalReportBody, _ctx: &mut Self::Context) -> Self::Result {
        self.alive_player_count = self.alive_player_count - 1;
        self.players.get(&msg.corpse).unwrap().do_send(msg);
    }
}

impl Handler<ReportBodyValidated> for Game {
    type Result = ();
    fn handle(&mut self, msg: ReportBodyValidated, ctx: &mut Self::Context) -> Self::Result {
        self.send_message_to_all_users(OutgoingWebsocketMessage::BodyReported(BodyReported {
            corpse: msg.corpse,
            initiator: msg.initiator,
        }));
        self.start_meeting(ctx);
    }
}

impl Handler<PlayerInvalidAction> for Game {
    type Result = ();
    fn handle(&mut self, msg: PlayerInvalidAction, _ctx: &mut Self::Context) -> Self::Result {
        self.players
            .get(&msg.id)
            .unwrap()
            .do_send(OutgoingWebsocketMessage::InvalidAction(msg.error));
    }
}

impl Handler<GetUUID> for Game {
    type Result = Arc<Uuid>;
    fn handle(&mut self, _msg: GetUUID, _ctx: &mut Self::Context) -> Self::Result {
        Arc::new(Uuid::from_bytes(self.rng.gen())).clone()
    }
}

impl Handler<GetKillCooldown> for Game {
    type Result = Arc<Duration>;
    fn handle(&mut self, _msg: GetKillCooldown, _ctx: &mut Self::Context) -> Self::Result {
        Arc::new(self.kill_cooldown)
    }
}

impl Handler<ForwardedOutgoingWebsocketMessage> for Game {
    type Result = ();
    fn handle(
        &mut self,
        msg: ForwardedOutgoingWebsocketMessage,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.players.get(&msg.destination).unwrap().do_send(msg.msg);
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, _msg: StartGame, _ctx: &mut Self::Context) -> Self::Result {
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
                .do_send(InternalSetPlayerRole {
                    role: role.1,
                    kill_cooldown: self.kill_cooldown,
                });
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
