use actix::Actor;
use actix_test::TestServer;
use actix_web::{http::header::ContentType, test, App};
use among_us_server::config_app;
use among_us_server::game::Game;
use among_us_server::incoming_websocket_messages::*;
use among_us_server::outgoing_websocket_messages::*;
use among_us_server::player::Player;
use among_us_server::player::PlayerSerializable;
use awc::Client;
use futures_util::{SinkExt as _, StreamExt as _};
use test_fixtures::assert_connection_recieves_message;

#[test]
async fn responds_hi() {
    let app = test_fixtures::get_test_service().await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .uri("/hello")
        .to_request();
    let resp = test::call_and_read_body(&app, req).await;
    assert_eq!(resp, "Hi");
}

#[test]
async fn player_joins_game() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let _assigned_id_frame = connection.next().await.unwrap().unwrap();
    let join_message_frame = connection.next().await.unwrap().unwrap();

    let join_message = test_fixtures::get_websocket_frame_data(join_message_frame).unwrap();

    match join_message {
        OutgoingWebsocketMessage::PlayerStatus(status) => {
            assert_eq!(status.status, PlayerConnectionStatus::New);
            assert_eq!(status.player.username, "Chris");
        }
        _ => unreachable!("Parsed to wrong thing {}", format!("{:?}", join_message)),
    }
}

#[test]
async fn one_player_assigned_imposter() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/start-game").send().await;

    let player_id = test_fixtures::get_next_id_in_connection(&mut connection).await;

    let game_started = OutgoingWebsocketMessage::GameState(GameState {
        state: GameState::InGame,
    });

    let assigned_imposter = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Imposter,
        id: player_id,
    });

    assert_connection_recieves_message(&mut connection, assigned_imposter).await;
    assert_connection_recieves_message(&mut connection, game_started).await;
}

#[test]
async fn one_player_each_role() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut chris_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut kate_connection) = Client::new()
        .ws(server.url("/join-game?username=Kate"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/start-game").send().await;

    let chris_id = test_fixtures::get_next_id_in_connection(&mut chris_connection).await;
    let kate_id = test_fixtures::get_next_id_in_connection(&mut chris_connection).await;

    let chris_assigned_crewmate = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Crewmate,
        id: chris_id,
    });

    let kate_assigned_imposter = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Imposter,
        id: kate_id,
    });

    assert_connection_recieves_message(&mut chris_connection, chris_assigned_crewmate).await;
    assert_connection_recieves_message(&mut kate_connection, kate_assigned_imposter).await;
}

#[test]
async fn other_player_receives_disconnect() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut chris_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut kate_connection) = Client::new()
        .ws(server.url("/join-game?username=Kate"))
        .connect()
        .await
        .unwrap();

    kate_connection.close().await.unwrap();

    let _chris_id = test_fixtures::get_next_id_in_connection(&mut chris_connection).await;
    let kate_id = test_fixtures::get_next_id_in_connection(&mut chris_connection).await;

    let kate_disconnect = OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
        player: PlayerSerializable::generate_for_user(&Player::new("Kate", kate_id), &kate_id),
        status: PlayerConnectionStatus::Disconnected,
    });
    assert_connection_recieves_message(&mut chris_connection, kate_disconnect).await;
}

#[test]
async fn imposter_kills_sucessfully_and_gets_reported() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut crewmate_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut imposter_connection) = Client::new()
        .ws(server.url("/join-game?username=Kate"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut second_crewmate_connection) = Client::new()
        .ws(server.url("/join-game?username=Ski"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/start-game").send().await;

    let crewmate_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;
    let _imposter_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;
    let second_crewmate_id =
        test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;

    imposter_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::KillPlayer(KillPlayer {
                target: crewmate_id,
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    second_crewmate_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::ReportBody(ReportBody {
                corpse: crewmate_id,
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    let body_reported_message = OutgoingWebsocketMessage::BodyReported(BodyReported {
        corpse: crewmate_id,
        initiator: second_crewmate_id,
    });

    assert_connection_recieves_message(&mut second_crewmate_connection, body_reported_message)
        .await;
}

#[test]
async fn imposter_kills_sucessfully_and_ends_game() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut crewmate_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut imposter_connection) = Client::new()
        .ws(server.url("/join-game?username=Kate"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/start-game").send().await;

    let crewmate_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;
    let imposter_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;

    imposter_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::KillPlayer(KillPlayer {
                target: crewmate_id,
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    let imposter_successful_kill = OutgoingWebsocketMessage::SuccessfulKill(crewmate_id);
    assert_connection_recieves_message(&mut imposter_connection, imposter_successful_kill).await;

    let crewmate_death = OutgoingWebsocketMessage::PlayerDied(PlayerDied {
        killer: imposter_id,
    });
    assert_connection_recieves_message(&mut crewmate_connection, crewmate_death).await;

    let imposters_won = OutgoingWebsocketMessage::GameOver(Winner::Imposters);
    assert_connection_recieves_message(&mut imposter_connection, imposters_won).await;
}

#[test]
async fn crewmate_votes_out_imposter_and_ends_game() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut crewmate_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let (_resp, mut imposter_connection) = Client::new()
        .ws(server.url("/join-game?username=Kate"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/start-game").send().await;

    let _crewmate_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;
    let imposter_id = test_fixtures::get_next_id_in_connection(&mut crewmate_connection).await;

    let _ = server.post("/start-meeting").send().await;

    crewmate_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::Vote(Vote {
                target: Some(imposter_id),
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    imposter_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::Vote(Vote {
                target: Some(imposter_id),
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    let imposter_voted_out_msg = OutgoingWebsocketMessage::VotingResults(VotingResults {
        ejected_player: Some(imposter_id),
    });

    assert_connection_recieves_message(&mut imposter_connection, imposter_voted_out_msg).await;

    let crewmates_won_message = OutgoingWebsocketMessage::GameOver(Winner::Crewmates);
    assert_connection_recieves_message(&mut imposter_connection, crewmates_won_message).await;
}

#[test]
async fn player_changes_color() {
    let server = test_fixtures::get_test_server();

    let (_resp, mut connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let player_id = test_fixtures::get_next_id_in_connection(&mut connection).await;

    let route = format!("get-player-color?id={player_id}");
    println!("the route is {route}");

    let mut response = server.get(route.clone()).send().await.unwrap();
    println!("got response: {:?}", response);

    let color_from_server = String::from_utf8(response.body().await.unwrap().to_vec()).unwrap();

    assert_eq!(color_from_server, "#FFFFFF".to_string());

    let color = "#ABABAB".to_string();

    connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::ChooseColor(ChooseColor {
                color: color.clone(),
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    let color_from_server = String::from_utf8(
        server
            .get(route)
            .send()
            .await
            .unwrap()
            .body()
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    assert_eq!(color_from_server, color);
}

#[test]
async fn reset_game_works_basic() {
    let server: TestServer = test_fixtures::get_test_server();
    let (_resp, mut chris_connection) = Client::new()
        .ws(server.url("/join-game?username=Chris"))
        .connect()
        .await
        .unwrap();

    let _ = server.post("/reset-game").send().await;

    let game_reset_msg = OutgoingWebsocketMessage::GameState(GameState {
        state: GameState::Reset,
    });

    assert_connection_recieves_message(&mut chris_connection, game_reset_msg).await;
}

mod test_fixtures {
    use actix_codec::Framed;
    use actix_http::ws::{Codec, Frame};
    use actix_http::Request;
    use actix_test;
    use actix_web::body::BoxBody;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::error::Error;
    use among_us_server::game::GameSettings;
    use among_us_server::outgoing_websocket_messages::OutgoingWebsocketMessage;
    use awc::BoxedSocket;
    use std::string::String;
    use std::time::Duration;
    use uuid::Uuid;

    use super::*;

    pub async fn get_test_service(
    ) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = Error> {
        let settings = GameSettings {
            kill_cooldown: Duration::from_secs(0),
        };
        let game = Game::new(settings, 0).start();
        test::init_service(App::new().configure(config_app(game.clone()))).await
    }

    pub fn get_test_server() -> actix_test::TestServer {
        let settings = GameSettings {
            kill_cooldown: Duration::from_secs(0),
        };
        let game = Game::new(settings, 0).start();
        actix_test::start(move || App::new().configure(config_app(game.clone())))
    }

    pub fn get_websocket_frame_data(frame: Frame) -> Option<OutgoingWebsocketMessage> {
        match frame {
            Frame::Text(t) => {
                let bytes = t.as_ref();
                let data = String::from_utf8(bytes.to_vec()).unwrap();
                let value: OutgoingWebsocketMessage = serde_json::from_str(&data).unwrap();
                return Some(value);
            }
            _ => return None,
        }
    }
    pub async fn get_next_id_in_connection(connection: &mut Framed<BoxedSocket, Codec>) -> Uuid {
        while let Some(message) =
            get_websocket_frame_data(connection.next().await.unwrap().unwrap())
        {
            if let OutgoingWebsocketMessage::PlayerStatus(status) = message {
                return status.player.id;
            }
        }
        unreachable!("{}", format!("Unable to find next id in connection",));
    }
    pub async fn assert_connection_recieves_message(
        connection: &mut Framed<BoxedSocket, Codec>,
        desired_message: OutgoingWebsocketMessage,
    ) -> () {
        while let Some(message) =
            get_websocket_frame_data(connection.next().await.unwrap().unwrap())
        {
            if message == desired_message {
                return;
            }
            match (&message, &desired_message) {
                (
                    OutgoingWebsocketMessage::PlayerStatus(player_status),
                    OutgoingWebsocketMessage::PlayerStatus(desired_player_status),
                ) => {
                    if player_status.player.username == desired_player_status.player.username
                        && player_status.player.id == desired_player_status.player.id
                        && player_status.player.color == desired_player_status.player.color
                        && player_status.status == desired_player_status.status
                    {
                        return;
                    }
                }
                _ => {}
            }
        }
        assert!(
            false,
            "{}",
            format!(
                "Unable to find desired message in connection: {:#?}",
                desired_message
            )
        )
    }
}
