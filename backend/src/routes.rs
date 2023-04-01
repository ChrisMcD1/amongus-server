use crate::game::*;
use crate::internal_messages::*;
use crate::player::PlayerWebsocket;
use actix::prelude::*;
use actix_web::cookie::Cookie;
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
    println!("User made request: {:#?}", req);
    let game_has_started = game.send(HasGameStarted {}).await.unwrap();
    if game_has_started {
        return error::ErrorBadRequest("Game has already begun! You cannot join").into();
    }
    if params.username.is_empty() {
        return error::ErrorBadRequest("You must input a name ;)").into();
    }
    let player_id = game.send(GetNextUUID {}).await.unwrap();

    let domain = if cfg!(debug_assertions) {
        "localhost"
    } else {
        ".amongus-irl.com"
    };

    let cookie = Cookie::build("player_id", player_id.clone().to_string())
        .domain(domain)
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .finish();

    game.do_send(RegisterPlayer {
        name: params.username.clone(),
        id: *player_id,
    });

    let player_websocket = PlayerWebsocket::new(*player_id, game.get_ref().clone());
    let mut player_websocket_active = ws::start(player_websocket, &req, stream).unwrap();
    player_websocket_active.add_cookie(&cookie).unwrap();

    player_websocket_active
}

#[derive(Deserialize)]
pub struct PlayerRejoinParams {
    id: Uuid,
}

#[get("player-exists")]
pub async fn player_exists(
    _req: HttpRequest,
    _stream: Payload,
    params: Query<PlayerRejoinParams>,
    game: Data<Addr<Game>>,
) -> impl Responder {
    let player_exists = game.send(PlayerExists { id: params.id }).await.unwrap();

    player_exists.to_string()
}

#[get("/player-rejoin-game")]
pub async fn player_rejoin(
    req: HttpRequest,
    stream: Payload,
    params: Query<PlayerRejoinParams>,
    game: Data<Addr<Game>>,
) -> impl Responder {
    let player_websocket = PlayerWebsocket::new(params.id, game.get_ref().clone());
    ws::start(player_websocket, &req, stream).unwrap()
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
    "Resetting game state and disconnecting all players!"
}
