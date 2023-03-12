use crate::incoming_websocket_messages::IncomingWebsocketMessage;
use crate::internal_messages::*;
use crate::outgoing_websocket_messages::OutgoingWebsocketMessage;
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
    players: BTreeMap<Uuid, Player>,
    kill_cooldown: Duration,
    pub rng: Pcg32,
    meeting: Option<Meeting>,
}

#[derive(Debug)]
pub struct GameSettings {
    pub kill_cooldown: Duration,
}

impl Game {
    fn send_message_to_all_users(&mut self, msg: OutgoingWebsocketMessage) {
        self.players
            .iter_mut()
            .for_each(|player| player.1.send_outgoing_message(msg.clone()))
    }
    pub fn new(settings: GameSettings, seed: u64) -> Self {
        Game {
            state: GameStateEnum::Lobby,
            players: BTreeMap::new(),
            kill_cooldown: settings.kill_cooldown,
            rng: Pcg32::seed_from_u64(seed),
            meeting: None,
        }
    }
    pub fn alive_players(&self) -> u32 {
        self.players.iter().filter(|player| player.1.alive).count() as u32
    }
    pub fn crewmates_alive(&self) -> u32 {
        self.players
            .iter()
            .filter(|player| player.1.role.unwrap() == Role::Crewmate)
            .filter(|player| player.1.alive)
            .count() as u32
    }
    pub fn imposters_alive(&self) -> u32 {
        self.players
            .iter()
            .filter(|player| match player.1.role.unwrap() {
                Role::Imposter(_) => true,
                _ => false,
            })
            .filter(|player| player.1.alive)
            .count() as u32
    }
    pub fn get_imposters(&self) -> Vec<Player> {
        self.players
            .clone()
            .into_iter()
            .map(|player| player.1)
            .filter(|player| match player.role {
                Some(Role::Imposter(_)) => true,
                _ => false,
            })
            .collect()
    }
    pub fn start_meeting(&mut self, ctx: &mut Context<Self>) {
        self.meeting = Some(Meeting::new(self.alive_players()));
        println!("Started meeting as {:?}", self.meeting);
        ctx.notify_later(EndVoting {}, VOTING_TIME);
    }
    pub fn end_meeting(&mut self) {
        println!("Stopping meeting");
        {
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
                        let mut voted_out = self.players.get_mut(&voted_out_user).unwrap();
                        voted_out.alive = false;
                    }
                    self.notify_players_of_next_state();
                }
                None => {
                    println!("Received Message to end meeting, but it has already ended!")
                }
            }
        }
    }
    pub fn has_winner(&self) -> Option<Winner> {
        if self.crewmates_alive() == 0 {
            Some(Winner::Imposters)
        } else if self.imposters_alive() == 0 {
            Some(Winner::Crewmates)
        } else {
            None
        }
    }
    pub fn notify_players_of_next_state(&mut self) {
        match self.has_winner() {
            Some(winner) => {
                self.send_message_to_all_users(OutgoingWebsocketMessage::GameOver(winner))
            }
            None => {
                self.send_message_to_all_users(OutgoingWebsocketMessage::GameState(GameState {
                    state: GameStateEnum::InGame,
                }))
            }
        }
    }
    pub fn notify_others_about_this_player(
        &mut self,
        player_id: &Uuid,
        player_status: PlayerConnectionStatus,
    ) {
        let player = self.players.get_mut(player_id).unwrap();
        let player_status = OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
            player: player.clone(),
            status: player_status,
        });
        self.send_message_to_all_users(player_status);
    }
    pub fn notify_player_with_game_status(&mut self, player_id: &Uuid) {
        self.tell_player_about_others(player_id);
        self.tell_player_about_roles(player_id);
        let player = self.players.get_mut(player_id).unwrap();
        player.send_outgoing_message(OutgoingWebsocketMessage::GameState(GameState {
            state: self.state.clone(),
        }));
    }
    fn tell_player_about_others(&mut self, player_id: &Uuid) {
        let existing_players_status: Vec<OutgoingWebsocketMessage> = self
            .players
            .iter()
            .filter(|(_, existing_player)| existing_player.id != *player_id)
            .map(|(_, existing_player)| {
                return OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
                    player: existing_player.clone(),
                    status: PlayerConnectionStatus::Existing,
                });
            })
            .collect();

        let player = self.players.get_mut(player_id).unwrap();
        for existing_status in existing_players_status.into_iter() {
            player.send_outgoing_message(existing_status);
        }
    }
    fn tell_player_about_roles(&mut self, player_id: &Uuid) {
        let other_imposters = self.get_imposters();
        let player = self.players.get_mut(player_id).unwrap();
        let role = player.role.clone();
        if let Some(role) = role {
            let role_assignment = match role {
                Role::Imposter(_) => RoleAssignment::Imposter,
                Role::Crewmate => RoleAssignment::Crewmate,
            };
            player.send_outgoing_message(OutgoingWebsocketMessage::PlayerRole(SetRole {
                role: role_assignment,
                id: player.id,
            }));
            if let Role::Imposter(_) = role {
                for imposter in other_imposters {
                    player.send_outgoing_message(OutgoingWebsocketMessage::PlayerRole(SetRole {
                        role: RoleAssignment::Imposter,
                        id: imposter.id,
                    }))
                }
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
        println!("Someone sent an EndVoting Message");
        self.end_meeting();
    }
}

impl Handler<StartMeeting> for Game {
    type Result = ();
    fn handle(&mut self, _msg: StartMeeting, ctx: &mut Self::Context) -> Self::Result {
        self.start_meeting(ctx);
    }
}

impl Game {
    fn handle_vote(&mut self, initiator: Uuid, target: Option<Uuid>) {
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
        if self.players.get(&corpse_id).unwrap().alive {
            let initiating_player = self.players.get_mut(&initiator).unwrap();
            initiating_player.send_outgoing_message(OutgoingWebsocketMessage::InvalidAction(
                "You cannot report this body, they are alive!".to_string(),
            ));
            return;
        }
        self.send_message_to_all_users(OutgoingWebsocketMessage::BodyReported(BodyReported {
            corpse: corpse_id,
            initiator,
        }));
        self.start_meeting(ctx);
    }
    fn handle_emergency_meeting(&mut self, initiator: Uuid, ctx: &mut Context<Self>) {
        self.send_message_to_all_users(OutgoingWebsocketMessage::EmergencyMeetingCalled(
            EmergencyMeetingCalled { initiator },
        ));
        self.start_meeting(ctx)
    }
    fn handle_kill(&mut self, initiator: Uuid, target: Uuid) {
        let potential_error_message = self.validate_kill_can_happen(initiator, target);
        if let Some(error_message) = potential_error_message {
            let initiating_player = self.players.get_mut(&initiator).unwrap();
            initiating_player
                .send_outgoing_message(OutgoingWebsocketMessage::InvalidAction(error_message));
            return;
        }
        let initiating_player = self.players.get_mut(&initiator).unwrap();
        match initiating_player.role {
                Some(Role::Imposter(imposter)) => {
                    initiating_player.role = Some(Role::Imposter(imposter.reset_kill_cooldown()));
                    initiating_player
                        .send_outgoing_message(OutgoingWebsocketMessage::SuccessfulKill(()));
                }
                _ => unreachable!("We already validated that this kill can happen, which means the initiator is an imposter"),
            }
        let target_player = self.players.get_mut(&target).unwrap();
        target_player.alive = false;
        target_player.send_outgoing_message(OutgoingWebsocketMessage::PlayerDied(PlayerDied {
            killer: initiator,
        }));
        println!(
            "All killing has ended, the new game state is:\n {:#?}",
            self
        );
    }

    fn validate_kill_can_happen(&mut self, initiator: Uuid, target: Uuid) -> Option<String> {
        let initiating_player = self.players.get(&initiator).unwrap();
        match initiating_player.role.unwrap() {
            Role::Imposter(ref mut imposter) => {
                if !imposter.kill_is_off_cooldown() {
                    return Some(format!(
                        "You are not off kill cooldown yet. Try again in {:#?}",
                        imposter.cooldown_remaining()
                    ));
                }
                let target_player = self.players.get(&target).unwrap();
                match target_player.role.unwrap() {
                    Role::Crewmate => {
                        if !target_player.alive {
                            return Some(format!(
                                "You cannot kill {} since they are already dead",
                                target_player.username
                            ));
                        }
                        return None;
                    }
                    Role::Imposter(_) => {
                        return Some("You cannot kill a fellow imposter, silly".to_string());
                    }
                }
            }
            _ => {
                return Some(
                    "Good try, but you can only kill people if you are an imposter!".to_string(),
                );
            }
        };
    }
    fn get_player_connection_status(&self, player_id: &Uuid) -> Option<PlayerConnectionStatus> {
        let player = self.players.get(player_id)?;
        Some(match player.has_connected_previously {
            true => PlayerConnectionStatus::Reconnected,
            false => PlayerConnectionStatus::New,
        })
    }
}

impl Handler<IncomingMessageInternal> for Game {
    type Result = ();
    fn handle(&mut self, msg: IncomingMessageInternal, ctx: &mut Self::Context) -> Self::Result {
        println!("{:?} sent message {:?}", msg.initiator, msg.incoming);
        match msg.incoming {
            IncomingWebsocketMessage::KillPlayer(kill) => {
                self.handle_kill(msg.initiator, kill.target);
                self.notify_players_of_next_state();
            }
            IncomingWebsocketMessage::ReportBody(report) => {
                self.handle_report(msg.initiator, report.corpse, ctx);
            }
            IncomingWebsocketMessage::Vote(vote) => {
                self.handle_vote(msg.initiator, vote.target);
            }
            IncomingWebsocketMessage::CallEmergencyMeeting => {
                self.handle_emergency_meeting(msg.initiator, ctx)
            }
            IncomingWebsocketMessage::ChooseColor(choose_color) => {
                self.players
                    .get_mut(&msg.initiator)
                    .unwrap()
                    .set_color(choose_color.color.clone());
                self.send_message_to_all_users(OutgoingWebsocketMessage::PlayerStatus(
                    PlayerStatus {
                        player: self.players.get(&msg.initiator).unwrap().clone(),
                        status: PlayerConnectionStatus::Existing,
                    },
                ))
            }
        }
    }
}

impl Handler<GetPlayerColor> for Game {
    type Result = String;
    fn handle(&mut self, msg: GetPlayerColor, _ctx: &mut Self::Context) -> Self::Result {
        self.players.get(&msg.id).unwrap().color.clone()
    }
}

impl Handler<PlayerDisconnected> for Game {
    type Result = ();
    fn handle(&mut self, msg: PlayerDisconnected, _ctx: &mut Self::Context) -> Self::Result {
        match self.players.get_mut(&msg.id) {
            Some(player) => {
                player.close_websocket();
                let player_status = OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
                    player: player.clone(),
                    status: PlayerConnectionStatus::Disconnected,
                });
                self.send_message_to_all_users(player_status);
            }
            None => {
                println!("Tried to remove player with id {:?}, but they had already been removed somewhere else", msg.id);
            }
        }
    }
}

impl Handler<HasGameStarted> for Game {
    type Result = bool;
    fn handle(&mut self, _msg: HasGameStarted, _ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            GameStateEnum::Lobby => false,
            GameStateEnum::Reset => false,
            GameStateEnum::InGame => true,
        }
    }
}

impl Handler<RegisterPlayer> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayer, _ctx: &mut Self::Context) -> Self::Result {
        let player = Player::new(&msg.name, msg.id);
        println!("Created player {:#?}", player);
        self.players.insert(msg.id, player);
    }
}

impl Handler<RegisterPlayerWebsocket> for Game {
    type Result = ();
    fn handle(&mut self, msg: RegisterPlayerWebsocket, _ctx: &mut Self::Context) -> Self::Result {
        if let None = self.players.get(&msg.id) {
            panic!(
                "Cannot register player websocket for nonexistant player: {}",
                format!("{:?}", msg.id)
            )
        }
        let player_status = self.get_player_connection_status(&msg.id).unwrap();

        self.players
            .get_mut(&msg.id)
            .unwrap()
            .set_websocket_address(msg.websocket);

        self.notify_player_with_game_status(&msg.id);

        self.notify_others_about_this_player(&msg.id, player_status);
    }
}

impl Handler<PlayerExists> for Game {
    type Result = bool;
    fn handle(&mut self, msg: PlayerExists, _ctx: &mut Self::Context) -> Self::Result {
        self.players.get(&msg.id).is_some()
    }
}

impl Handler<GetNextUUID> for Game {
    type Result = Arc<Uuid>;
    fn handle(&mut self, _msg: GetNextUUID, _ctx: &mut Self::Context) -> Self::Result {
        Arc::new(Uuid::from_bytes(self.rng.gen())).clone()
    }
}

impl Handler<ResetGame> for Game {
    type Result = ();
    fn handle(&mut self, _msg: ResetGame, _ctx: &mut Self::Context) -> Self::Result {
        self.state = GameStateEnum::Lobby;
        self.send_message_to_all_users(OutgoingWebsocketMessage::GameState(GameState {
            state: GameStateEnum::Reset,
        }));
        for (_, player) in self.players.iter_mut() {
            player.close_websocket();
        }
        self.players.clear();
    }
}

impl Handler<StartGame> for Game {
    type Result = ();
    fn handle(&mut self, _msg: StartGame, _ctx: &mut Self::Context) -> Self::Result {
        if self.state != GameStateEnum::Lobby {
            println!(
                "Unable to start game for game that is not in lobby state.
                     This game is in state {:?}",
                self.state
            );
        }
        self.state = GameStateEnum::InGame;
        let player_count = self.players.len();
        let mut imposter_count = get_imposter_count(player_count);
        let mut imposters: HashSet<Uuid> = HashSet::new();
        let mut player_roles: BTreeMap<Uuid, RoleAssignment> = self
            .players
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

        player_roles.iter().for_each(|role| {
            self.players
                .get_mut(&role.0)
                .unwrap()
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

#[derive(Debug)]
pub struct Meeting {
    alive_player_count: u32,
    votes: BTreeMap<Uuid, Option<Uuid>>,
}

impl Meeting {
    pub fn new(alive_player_count: u32) -> Self {
        Meeting {
            alive_player_count,
            votes: BTreeMap::new(),
        }
    }
    pub fn add_vote(&mut self, vote_by: Uuid, vote_for: Option<Uuid>) {
        self.votes.insert(vote_by, vote_for);
    }
    pub fn all_players_voted(&self) -> bool {
        u32::try_from(self.votes.len()).expect("Shouldn't exceed a u32 number of players lol")
            == self.alive_player_count
    }
    pub fn person_voted_out(&self) -> Option<Uuid> {
        let vote_threshold = (f64::from(self.alive_player_count) / 2f64).floor() as u32 + 1;
        let mut votes_for_each: BTreeMap<Uuid, u32> = BTreeMap::new();
        for vote in self.votes.iter() {
            let vote_for_option = vote.1;
            if let Some(vote_for) = vote_for_option {
                match votes_for_each.get(vote_for) {
                    Some(votes) => votes_for_each.insert(*vote_for, votes + 1),
                    None => votes_for_each.insert(*vote_for, 1),
                };
            }
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
