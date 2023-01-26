use crate::incoming_websocket_messages::IncomingWebsocketMessage;
use crate::internal_messages::*;
use crate::outgoing_websocket_messages::*;
use crate::player::*;
use actix::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg32;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug)]
pub struct Game {
    state: GameStateEnum,
    players_with_websockets: BTreeMap<Uuid, RefCell<Player>>,
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
        self.players_with_websockets
            .iter()
            .for_each(|player| player.1.borrow().send_outgoing_message(msg.clone()))
    }
    pub fn new(kill_cooldown: Duration, seed: u64) -> Self {
        Game {
            state: GameStateEnum::Lobby,
            players_with_websockets: BTreeMap::new(),
            kill_cooldown,
            rng: Pcg32::seed_from_u64(seed),
            meeting: None,
        }
    }
    pub fn alive_player_count(&self) -> u32 {
        self.players_with_websockets
            .iter()
            .filter(|player| player.1.borrow().alive)
            .count() as u32
    }
    pub fn start_meeting(&mut self, ctx: &mut Context<Self>) {
        self.meeting = Some(Meeting::new(self.alive_player_count()));
        println!("Started meeting as {:?}", self.meeting);
        ctx.notify_later(EndVoting {}, VOTING_TIME);
    }
    pub fn end_meeting(&mut self) {
        println!("Stopping meeting");
        match self.meeting.as_ref() {
            Some(meeting) => {
                let voted_out_user_option = meeting.person_voted_out();
                self.meeting = None;
                self.send_message_to_all_users(OutgoingWebsocketMessage::VotingResults(
                    VotingResults {
                        ejected_player: voted_out_user_option,
                    },
                ));
                if let Some(voted_out_user) = voted_out_user_option {
                    let mut voted_out = self
                        .players_with_websockets
                        .get(&voted_out_user)
                        .unwrap()
                        .borrow_mut();
                    voted_out.alive = false;
                }
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

impl Game {
    fn handle_vote(&mut self, initiator: Uuid, target: Uuid) {
        match self.meeting.as_mut() {
            Some(meeting) => {
                meeting.add_vote(initiator, target);
                if meeting.all_players_voted() {
                    self.end_meeting();
                }
            }
            None => println!("Cannot vote without a meeting active!"),
        }
    }
    fn handle_report(&mut self, initiator: Uuid, corpse_id: Uuid, ctx: &mut Context<Self>) {
        {
            let initiating_player = self
                .players_with_websockets
                .get(&initiator)
                .unwrap()
                .borrow();
            let corpse = self
                .players_with_websockets
                .get(&corpse_id)
                .unwrap()
                .borrow();
            if !corpse.alive {
                initiating_player.send_outgoing_message(OutgoingWebsocketMessage::InvalidAction(
                    "You cannot report this body, they are alive!".to_string(),
                ));
                return;
            }
            self.send_message_to_all_users(OutgoingWebsocketMessage::BodyReported(BodyReported {
                corpse: corpse_id,
                initiator,
            }));
        }
        self.start_meeting(ctx);
    }
    fn handle_kill(&mut self, initiator: Uuid, target: Uuid) {
        let mut initiating_player = self
            .players_with_websockets
            .get(&initiator)
            .unwrap()
            .borrow_mut();
        let mut target_player = self
            .players_with_websockets
            .get(&target)
            .unwrap()
            .borrow_mut();
        match initiating_player.role.unwrap() {
            Role::Imposter(ref mut imposter) => {
                if !imposter.kill_is_off_cooldown() {
                    initiating_player.send_outgoing_message(
                        OutgoingWebsocketMessage::InvalidAction(format!(
                            "You are not off kill cooldown yet. Try again in {:#?}",
                            imposter.cooldown_remaining()
                        )),
                    );
                    return;
                }
                match target_player.role.unwrap() {
                    Role::Crewmate => {
                        if !target_player.alive {
                            initiating_player.send_outgoing_message(
                                OutgoingWebsocketMessage::InvalidAction(format!(
                                    "You cannot kill {} since they are already dead",
                                    target_player.name
                                )),
                            );
                            return;
                        }
                        initiating_player.role =
                            Some(Role::Imposter(imposter.reset_kill_cooldown()));
                        target_player.alive = false;
                        initiating_player
                            .send_outgoing_message(OutgoingWebsocketMessage::SuccessfulKill());
                        target_player.send_outgoing_message(OutgoingWebsocketMessage::PlayerDied(
                            PlayerDied { killer: initiator },
                        ))
                    }
                    Role::Imposter(_) => initiating_player.send_outgoing_message(
                        OutgoingWebsocketMessage::InvalidAction(
                            "You cannot kill a fellow imposter, silly".to_string(),
                        ),
                    ),
                }
            }
            _ => {
                initiating_player.send_outgoing_message(OutgoingWebsocketMessage::InvalidAction(
                    "Good try, but you can only kill people if you are an imposter!".to_string(),
                ));
            }
        };
    }
}

impl Handler<IncomingMessageInternal> for Game {
    type Result = ();
    fn handle(&mut self, msg: IncomingMessageInternal, ctx: &mut Self::Context) -> Self::Result {
        println!("{:?} sent message {:?}", msg.initiator, msg.incoming);
        match msg.incoming {
            IncomingWebsocketMessage::KillPlayer(kill) => {
                self.handle_kill(msg.initiator, kill.target);
            }
            IncomingWebsocketMessage::ReportBody(report) => {
                self.handle_report(msg.initiator, report.corpse, ctx);
            }
            IncomingWebsocketMessage::Vote(vote) => {
                self.handle_vote(msg.initiator, vote.target);
            }
        }
    }
}

impl Handler<PlayerWithWebsocketDisconnected> for Game {
    type Result = ();
    fn handle(
        &mut self,
        msg: PlayerWithWebsocketDisconnected,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let player = self
            .players_with_websockets
            .remove(&msg.id)
            .expect("Cannot remove player that doesn't exist");
        let player = player.borrow();
        self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            username: player.name.clone(),
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

impl Handler<RegisterPlayerWithWebsocket> for Game {
    type Result = ();
    fn handle(
        &mut self,
        msg: RegisterPlayerWithWebsocket,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let player = Player::new(&msg.name, msg.id);
        self.players_with_websockets
            .insert(msg.id, RefCell::new(player));
    }
}

impl Handler<RegisterPlayerWebsocket> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayerWebsocket, _ctx: &mut Self::Context) -> Self::Result {
        self.players_with_websockets
            .get_mut(&msg.id)
            .unwrap()
            .borrow_mut()
            .set_websocket_address(msg.websocket);
        let player = self.players_with_websockets.get(&msg.id).unwrap().borrow();
        self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            username: player.name.clone(),
            id: player.id,
            status: PlayerConnectionStatus::New,
        }));
    }
}

impl Handler<GetNextUUID> for Game {
    type Result = Arc<Uuid>;
    fn handle(&mut self, _msg: GetNextUUID, _ctx: &mut Self::Context) -> Self::Result {
        Arc::new(Uuid::from_bytes(self.rng.gen())).clone()
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, _msg: StartGame, _ctx: &mut Self::Context) -> Self::Result {
        self.state = GameStateEnum::InGame;
        let player_count = self.players_with_websockets.len();
        let mut imposter_count = get_imposter_count(player_count);
        let mut imposters: HashSet<Uuid> = HashSet::new();
        let mut player_roles: BTreeMap<Uuid, RoleAssignment> = self
            .players_with_websockets
            .iter()
            .map(|player| (player.0.clone(), RoleAssignment::Crewmate))
            .collect();
        while imposter_count > 0 {
            let imposter_index = self.rng.gen_range(0..player_count);
            let player = self
                .players_with_websockets
                .iter()
                .nth(imposter_index)
                .unwrap();
            if imposters.contains(player.0) {
                continue;
            }
            imposters.insert(player.0.clone());
            player_roles.insert(player.0.clone(), RoleAssignment::Imposter);
            imposter_count = imposter_count - 1;
        }

        player_roles.iter().for_each(|role| {
            self.players_with_websockets
                .get(&role.0)
                .unwrap()
                .borrow_mut()
                .set_role(*role.1, self.kill_cooldown)
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
