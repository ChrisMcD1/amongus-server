pub mod game;
pub mod incoming_websocket_messages;
pub mod internal_messages;
pub mod outgoing_websocket_messages;
pub mod player;
pub mod routes;

use crate::game::Game;
use crate::routes::*;
use actix::Addr;
use actix_web::web::Data;

pub fn config_app(game: Addr<Game>) -> Box<dyn Fn(&mut actix_web::web::ServiceConfig)> {
    Box::new(move |cfg: &mut actix_web::web::ServiceConfig| {
        cfg.app_data(Data::new(game.clone()))
            .service(hello_world)
            .service(join_game)
            .service(start_game)
            .service(reset_game)
            .service(start_meeting)
            .service(get_player_color);
    })
}
