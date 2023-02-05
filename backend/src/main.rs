use actix::Actor;
use actix_web::{App, HttpServer};
use among_us_server::config_app;
use among_us_server::game::{Game, GameSettings};
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
    HttpServer::new(move || App::new().configure(config_app(game.clone())))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
