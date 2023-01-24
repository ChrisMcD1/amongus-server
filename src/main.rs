use actix::Actor;
use actix_web::{web::Data, App, HttpServer};
use among_us_server::game::Game;
use among_us_server::routes::*;

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
