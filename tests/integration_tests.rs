use actix::Actor;
use actix_web::web::{self, Data};
use actix_web::{body, http};
use actix_web::{http::header::ContentType, test, App};
use among_us_server::game::Game;
use among_us_server::outgoing_websocket_messages::*;
use among_us_server::routes::*;
use awc::Client;
use futures_util::{SinkExt as _, StreamExt as _};

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

mod test_fixtures {
    use actix_http::ws::Frame;
    use actix_http::Request;
    use actix_test;
    use actix_web::body::BoxBody;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::error::Error;
    use among_us_server::outgoing_websocket_messages::OutgoingWebsocketMessage;

    use super::*;

    pub async fn get_test_service(
    ) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = Error> {
        let game = Game::new(0).start();
        test::init_service(
            App::new()
                .service(hello_world)
                .service(join_game)
                .service(start_game)
                .app_data(Data::new(game.clone())),
        )
        .await
    }

    pub fn get_test_server() -> actix_test::TestServer {
        let game = Game::new(0).start();
        actix_test::start(move || {
            App::new()
                .service(hello_world)
                .service(join_game)
                .service(start_game)
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
}
