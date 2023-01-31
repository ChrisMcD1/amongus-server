use actix::Actor;
use actix_web::{web::Data, App, HttpServer};
use among_us_server::game::{Game, GameSettings};
use among_us_server::routes::*;
use rand::prelude::*;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let game_settings = GameSettings {
        kill_cooldown: Duration::from_secs(60),
    };
    let game = Game::new(game_settings, rng.gen()).start();
    HttpServer::new(move || {
        App::new()
            .service(hello_world)
            .service(join_game)
            .service(start_game)
            .service(start_meeting)
            .service(get_player_color)
            .app_data(Data::new(game.clone()))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
