use crate::game::*;
use crate::game_messages::*;
use crate::player_websocket::PlayerWebsocket;
use crate::Player;
use actix::prelude::*;
use actix_web::post;
use actix_web::{
    error, get,
    web::{Data, Payload, Query},
    HttpRequest, Responder,
};
use serde::Deserialize;

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
    let player = Player::new(&params.username, game.get_ref().clone());
    let player_uuid = player.id.clone();
    let player_name = player.name.clone();
    let resp = ws::start(player, &req, stream).unwrap();

    game.do_send(PrintGameState {});

    resp
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
