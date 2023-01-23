use actix::Actor;
use actix_web::{web::Data, App, HttpServer};
use game::Game;
use routes::*;

use player::*;
mod game;
mod game_messages;
mod player;
mod player_messages;
mod player_websocket;
mod player_websocket_messages;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let game = Game::default().start();
    HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .service(join_game)
            .service(start_game)
            .app_data(Data::new(game.clone()))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
