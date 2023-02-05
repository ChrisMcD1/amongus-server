use crate::game::*;
use crate::internal_messages::*;
use crate::player::PlayerWebsocket;
use actix::prelude::*;
use actix_web::post;
use actix_web::{
    error, get,
    web::{Data, Payload, Query},
    HttpRequest, Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use actix_web_actors::ws;
#[get("/hello")]
pub async fn hello_world() -> impl Responder {
    "Hi"
}

#[derive(Deserialize)]
pub struct JoinGameParams {
    username: String,
}

#[get("/join-game")]
pub async fn join_game(
    req: HttpRequest,
    stream: Payload,
    params: Query<JoinGameParams>,
    game: Data<Addr<Game>>,
) -> impl Responder {
    let game_has_started = game.send(HasGameStarted {}).await.unwrap();
    if game_has_started {
        return error::ErrorBadRequest("Game has already begun! You cannot join").into();
    }
    let player_id = game.send(GetNextUUID {}).await.unwrap();

    game.do_send(RegisterPlayer {
        name: params.username.clone(),
        id: *player_id,
    });

    let player_websocket = PlayerWebsocket::new(*player_id, game.get_ref().clone());
    let player_websocket_active = ws::start(player_websocket, &req, stream).unwrap();

    player_websocket_active
}

#[post("/start-game")]
pub async fn start_game(
    _req: HttpRequest,
    _stream: Payload,
    game: Data<Addr<Game>>,
) -> impl Responder {
    game.do_send(StartGame {});
    "Game Started"
}

#[post("/start-meeting")]
pub async fn start_meeting(
    _req: HttpRequest,
    _stream: Payload,
    game: Data<Addr<Game>>,
) -> impl Responder {
    game.do_send(StartMeeting {});
    "Meeting Started!"
}

#[derive(Deserialize)]
pub struct GetPlayerColorParams {
    id: Uuid,
}

#[get("/get-player-color")]
pub async fn get_player_color(
    _req: HttpRequest,
    _stream: Payload,
    params: Query<GetPlayerColorParams>,
    game: Data<Addr<Game>>,
) -> impl Responder {
    let color = game.send(GetPlayerColor { id: params.id }).await.unwrap();
    color
}

#[post("/reset-game")]
pub async fn reset_game(
    _req: HttpRequest,
    _stream: Payload,
    game: Data<Addr<Game>>,
) -> impl Responder {
    game.do_send(ResetGame {});
    "Game Started"
}
