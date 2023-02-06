use actix::Actor;
use actix_test::TestServer;
use actix_web::{http::header::ContentType, test, App};
use among_us_server::config_app;
use among_us_server::game::Game;
use among_us_server::incoming_websocket_messages::*;
use among_us_server::outgoing_websocket_messages::*;
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

    let join_message_frame = connection.next().await.unwrap().unwrap();

    let join_message = test_fixtures::get_websocket_frame_data(join_message_frame).unwrap();

    match join_message {
        OutgoingWebsocketMessage::PlayerStatus(status) => {
            assert_eq!(status.status, PlayerConnectionStatus::New);
            assert_eq!(status.username, "Chris");
        }
        _ => assert!(false, "Parsed to wrong thing"),
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

    let _player_join = connection.next().await;

    let game_started = OutgoingWebsocketMessage::GameState(GameState {
        state: GameStateEnum::InGame,
    });

    let assigned_imposter = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Imposter,
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

    let _chris_join = chris_connection.next().await;
    let _kate_join = chris_connection.next().await;

    let _kate_join = kate_connection.next().await;

    let chris_assigned_crewmate = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Crewmate,
    });

    let kate_assigned_imposter = OutgoingWebsocketMessage::PlayerRole(SetRole {
        role: RoleAssignment::Imposter,
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

    let _chris_join = chris_connection.next().await;
    let kate_join_frame = chris_connection.next().await.unwrap().unwrap();
    let kate_join = test_fixtures::get_websocket_frame_data(kate_join_frame).unwrap();

    let kate_id = match kate_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

    let kate_disconnect = OutgoingWebsocketMessage::PlayerStatus(PlayerStatus {
        username: "Kate".to_string(),
        id: kate_id,
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

    let crewmate_join = crewmate_connection.next().await.unwrap().unwrap();
    let _imposter_join = crewmate_connection.next().await.unwrap().unwrap();
    let second_crewmate_join = crewmate_connection.next().await.unwrap().unwrap();

    let crewmate_join = test_fixtures::get_websocket_frame_data(crewmate_join).unwrap();
    let second_crewmate_join =
        test_fixtures::get_websocket_frame_data(second_crewmate_join).unwrap();

    let crewmate_id = match crewmate_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

    let second_crewmate_id = match second_crewmate_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

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

    let crewmate_join = crewmate_connection.next().await.unwrap().unwrap();
    let imposter_join = crewmate_connection.next().await.unwrap().unwrap();

    let crewmate_join = test_fixtures::get_websocket_frame_data(crewmate_join).unwrap();
    let imposter_join = test_fixtures::get_websocket_frame_data(imposter_join).unwrap();

    let crewmate_id = match crewmate_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

    let imposter_id = match imposter_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

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

    let imposter_successful_kill = OutgoingWebsocketMessage::SuccessfulKill();
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

    let _crewmate_join = crewmate_connection.next().await.unwrap().unwrap();
    let imposter_join = crewmate_connection.next().await.unwrap().unwrap();

    let imposter_join = test_fixtures::get_websocket_frame_data(imposter_join).unwrap();

    let imposter_id = match imposter_join {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

    let _ = server.post("/start-meeting").send().await;

    crewmate_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::Vote(Vote {
                target: imposter_id,
            }))
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();

    imposter_connection
        .send(awc::ws::Message::Text(
            serde_json::to_string(&IncomingWebsocketMessage::Vote(Vote {
                target: imposter_id,
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

    let join_message_frame = connection.next().await.unwrap().unwrap();
    let join_message = test_fixtures::get_websocket_frame_data(join_message_frame).unwrap();

    let player_id = match join_message {
        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
        _ => unreachable!(),
    };

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

    let _join = chris_connection.next().await;

    let no_message = chris_connection.next().await;

    let websocket_disconnected = no_message.is_none();

    assert_eq!(websocket_disconnected, true);
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
