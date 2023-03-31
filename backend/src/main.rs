use actix::Actor;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use among_us_server::config_app;
use among_us_server::game::{Game, GameSettings};
use rand::prelude::*;
use rustls::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let (full_chain_path, private_key_path) = {
        if cfg!(debug_assertions) {
            ("localhost.crt", "localhost.key")
        } else {
            ("/keys/fullchain.pem", "/keys/privkey.pem")
        }
    };
    let cert_file = match File::open(full_chain_path) {
        Ok(file) => file,
        _ => {
            unreachable!("Unable to find full chain path at {:?}", full_chain_path)
        }
    };

    let cert_reader = &mut BufReader::new(cert_file);
    let cert_chain = certs(cert_reader)
        .unwrap()
        .into_iter()
        .map(|vec| rustls::Certificate(vec))
        .collect();

    let private_key_file = match File::open(private_key_path) {
        Ok(file) => file,
        _ => {
            unreachable!(
                "Unable to find private key chain path at {:?}",
                private_key_path
            )
        }
    };

    let private_key_reader = &mut BufReader::new(private_key_file);
    let private_key = pkcs8_private_keys(private_key_reader).unwrap().remove(0);

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, rustls::PrivateKey(private_key))
        .unwrap();

    let mut rng = rand::thread_rng();
    let game_settings = GameSettings {
        kill_cooldown: Duration::from_secs(0),
    };
    let game = Game::new(game_settings, rng.gen()).start();
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header();
        App::new().configure(config_app(game.clone())).wrap(cors)
    })
    .bind_rustls("localhost:9090", config)?
    .run()
    .await
}
