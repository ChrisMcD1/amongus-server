use std::time::Duration;

use actix::Actor;
use actix_web::{web::Data, App, HttpServer};
use among_us_server::game::Game;
use among_us_server::routes::*;
use rand::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let game = Game::new(Duration::from_secs(60), rng.gen()).start();
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
