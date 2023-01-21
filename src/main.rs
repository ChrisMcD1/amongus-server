use crate::game::*;
use crate::game_messages::*;
use crate::player_messages::*;
use crate::player_websocket::PlayerWebsocket;
use actix::prelude::*;
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Payload, Query},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

use actix_web_actors::ws;
use player::*;
use serde::Deserialize;
mod game;
mod game_messages;
mod player;
mod player_messages;
mod player_websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let game = Game::default().start();
    HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .service(join_game)
            .data(game.clone())
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hi"
}

#[derive(Deserialize)]
pub struct JoinGameParams {
    username: String,
}

#[get("/join-game")]
async fn join_game(
    req: HttpRequest,
    stream: Payload,
    params: Query<JoinGameParams>,
    game: Data<Addr<Game>>,
) -> impl Responder {
    let player = Player::new(&params.username, game.get_ref().clone());
    let player_uuid = player.uuid.clone();
    let player_name = player.name.clone();
    let player = player.start();

    let player_websocket = PlayerWebsocket::new(player.clone());

    let resp = ws::start(player_websocket, &req, stream).unwrap();

    game.do_send(RegisterPlayer {
        uuid: player_uuid,
        name: player_name,
        player,
    });

    game.do_send(PrintGameState {});

    resp
}
