use actix::Actor;
use actix_web::web::Data;
use actix_web::{http::header::ContentType, test, App};
use among_us_server::game::Game;
use among_us_server::incoming_websocket_messages::*;
use among_us_server::outgoing_websocket_messages::*;
use among_us_server::routes::*;
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

    let role_assigned_frame = connection.next().await.unwrap().unwrap();
    let game_started_frame = connection.next().await.unwrap().unwrap();

    let game_started = test_fixtures::get_websocket_frame_data(game_started_frame).unwrap();
    let role_assigned = test_fixtures::get_websocket_frame_data(role_assigned_frame).unwrap();

    match game_started {
        OutgoingWebsocketMessage::GameState(game_state) => {
            assert_eq!(game_state.state, GameStateEnum::InGame);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }

    match role_assigned {
        OutgoingWebsocketMessage::PlayerRole(player_role) => {
            assert_eq!(player_role.role, RoleAssignment::Imposter);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }
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

    let chris_role_assigned_frame = chris_connection.next().await.unwrap().unwrap();
    let kate_role_assigned_frame = kate_connection.next().await.unwrap().unwrap();

    let chris_role_assigned =
        test_fixtures::get_websocket_frame_data(chris_role_assigned_frame).unwrap();
    let kate_role_assigned =
        test_fixtures::get_websocket_frame_data(kate_role_assigned_frame).unwrap();

    match chris_role_assigned {
        OutgoingWebsocketMessage::PlayerRole(player_role) => {
            assert_eq!(player_role.role, RoleAssignment::Crewmate);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }

    match kate_role_assigned {
        OutgoingWebsocketMessage::PlayerRole(player_role) => {
            assert_eq!(player_role.role, RoleAssignment::Imposter);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }
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
    let _kate_join = chris_connection.next().await;

    let kate_disconnect_frame = chris_connection.next().await.unwrap().unwrap();

    let kate_disconnect = test_fixtures::get_websocket_frame_data(kate_disconnect_frame).unwrap();

    match kate_disconnect {
        OutgoingWebsocketMessage::PlayerStatus(player_status) => {
            assert_eq!(player_status.username, "Kate");
            assert_eq!(player_status.status, PlayerConnectionStatus::Disconnected);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }
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
    let imposter_join = crewmate_connection.next().await.unwrap().unwrap();
    let _second_crewmate_join = crewmate_connection.next().await;
    let _crewmate_role_assign = crewmate_connection.next().await;
    let _crewmate_game_start = crewmate_connection.next().await;
    let _imposter_join = imposter_connection.next().await;
    let _imposter_role_assign = imposter_connection.next().await;
    let _second_crewmate_join = imposter_connection.next().await;
    let _imposter_game_start = imposter_connection.next().await;
    let _second_crewmate_join = second_crewmate_connection.next().await;
    let _second_crewmate_role_assign = second_crewmate_connection.next().await;
    let _second_crewmate_game_start = second_crewmate_connection.next().await;

    let crewmate_join = test_fixtures::get_websocket_frame_data(crewmate_join).unwrap();
    let imposter_join = test_fixtures::get_websocket_frame_data(imposter_join).unwrap();

    let crewmate_id = match crewmate_join {
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

    let meeting_began_frame = second_crewmate_connection.next().await.unwrap().unwrap();
    let meeting_began = test_fixtures::get_websocket_frame_data(meeting_began_frame).unwrap();

    match meeting_began {
        OutgoingWebsocketMessage::BodyReported(body_reported) => {
            assert_eq!(body_reported.corpse, crewmate_id);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }
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
    let _crewmate_role_assign = crewmate_connection.next().await;
    let _crewmate_game_start = crewmate_connection.next().await;
    let _imposter_join = imposter_connection.next().await;
    let _imposter_role_assign = imposter_connection.next().await;
    let _imposter_game_start = imposter_connection.next().await;

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

    let imposter_success_frame = imposter_connection.next().await.unwrap().unwrap();
    let crewmate_death_frame = crewmate_connection.next().await.unwrap().unwrap();

    let imposter_success = test_fixtures::get_websocket_frame_data(imposter_success_frame).unwrap();
    let crewmate_death = test_fixtures::get_websocket_frame_data(crewmate_death_frame).unwrap();

    match imposter_success {
        OutgoingWebsocketMessage::SuccessfulKill() => {
            assert!(true, "We did it bois");
        }
        _ => assert!(
            false,
            "{}",
            format!("Parsed to wrong thing: {:?}", imposter_success)
        ),
    }

    match crewmate_death {
        OutgoingWebsocketMessage::PlayerDied(player_died) => {
            assert_eq!(player_died.killer, imposter_id);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }

    let imposters_won_frame = imposter_connection.next().await.unwrap().unwrap();
    let imposter_won = test_fixtures::get_websocket_frame_data(imposters_won_frame).unwrap();

    match imposter_won {
        OutgoingWebsocketMessage::GameOver(winner) => {
            assert_eq!(winner, Winner::Imposters);
        }
        _ => assert!(false, "Parsed to wrong thing"),
    }
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

    let imposter_join = imposter_connection.next().await.unwrap().unwrap();

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
#[ignore]
async fn player_changes_color() {
    //    let server = test_fixtures::get_test_server();
    //
    //    let (_resp, mut connection) = Client::new()
    //        .ws(server.url("/join-game?username=Chris"))
    //        .connect()
    //        .await
    //        .unwrap();
    //
    //    let join_message_frame = connection.next().await.unwrap().unwrap();
    //    let join_message = test_fixtures::get_websocket_frame_data(join_message_frame).unwrap();
    //
    //    let player_id = match join_message {
    //        OutgoingWebsocketMessage::PlayerStatus(status) => status.id,
    //        _ => unreachable!(),
    //    };
    //
    //    let route = format!("/get-player-color?id={player_id}");
    //    println!("the route is {route}");
    //    let return_body = server
    //        .get(route)
    //        .send()
    //        .await
    //        .unwrap()
    //        .body()
    //        .await
    //        .unwrap()
    //        .to_vec();
    //    //    println!("return body is {return_body}");
    //
    //    let color_from_server = String::from_utf8(
    //        server
    //            .get(route)
    //            .send()
    //            .await
    //            .unwrap()
    //            .body()
    //            .await
    //            .unwrap()
    //            .to_vec(),
    //    )
    //    .unwrap();
    //
    //    assert_eq!(color_from_server, "#FFFFFF".to_string());

    //    let color = "#ABABAB".to_string();
    //
    //    connection
    //        .send(awc::ws::Message::Text(
    //            serde_json::to_string(&IncomingWebsocketMessage::ChooseColor(ChooseColor {
    //                color: color.clone(),
    //            }))
    //            .unwrap()
    //            .into(),
    //        ))
    //        .await
    //        .unwrap();
    //
    //    let route = format!("/get-player-color?id={player_id}");
    //    println!("the route is {route}");
    //
    //    let color_from_server = String::from_utf8(
    //        server
    //            .get(route)
    //            .send()
    //            .await
    //            .unwrap()
    //            .body()
    //            .await
    //            .unwrap()
    //            .to_vec(),
    //    )
    //    .unwrap();
    //
    //    assert_eq!(color_from_server, color);
}

mod test_fixtures {
    use actix_codec::Framed;
    use actix_http::h1::ClientCodec;
    use actix_http::ws::{Codec, Frame};
    use actix_http::Request;
    use actix_test;
    use actix_web::body::BoxBody;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::error::Error;
    use among_us_server::outgoing_websocket_messages::OutgoingWebsocketMessage;
    use awc::BoxedSocket;
    use std::string::String;
    use std::time::Duration;

    use super::*;

    pub async fn get_test_service(
    ) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = Error> {
        let game = Game::new(Duration::from_secs(0), 0).start();
        test::init_service(
            App::new()
                .service(hello_world)
                .service(join_game)
                .service(start_game)
                .service(start_meeting)
                .app_data(Data::new(game.clone())),
        )
        .await
    }

    pub fn get_test_server() -> actix_test::TestServer {
        let game = Game::new(Duration::from_secs(0), 0).start();
        actix_test::start(move || {
            App::new()
                .service(hello_world)
                .service(join_game)
                .service(start_game)
                .service(start_meeting)
                .app_data(Data::new(game.clone()))
        })
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
