use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware};
use anyhow::{anyhow, Result};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Tosca REST-backend")]
struct Arguments {
    /// The config file (in TOML format)
    config_file: String,
}

#[derive(Deserialize)]
struct Config {
    public_host: String,
    port: u16,
    certificate_file: String,
    key_file: String,
    login_provider: String,
    user_provider: String,
}

fn parse_config(cfg_file: &str) -> Result<Config> {
    let f_data = std::fs::read_to_string(cfg_file)?;
    toml::from_str(&f_data).map_err(|x| anyhow!("Failed to load config file: {:?}", x))
}

fn load_ssl_keys(config: &Config) -> ServerConfig {
    let mut cfg = ServerConfig::new(NoClientAuth::new());

    let mut cert_file = BufReader::new(File::open(&config.certificate_file).unwrap());
    let mut key_file = BufReader::new(File::open(&config.key_file).unwrap());

    let cert_chain = certs(&mut cert_file).unwrap();
    let mut keys = pkcs8_private_keys(&mut key_file).unwrap();

    cfg.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    cfg
}

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

use web_backend::api;
use db_connector::create_db_pool_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();

    let config = parse_config(&args.config_file).unwrap();
    let cfg = load_ssl_keys(&config);

    pretty_env_logger::init();

    // Uses DATBASE_URL, from environment or .env file
    let db_pool = create_db_pool_env()
        .map_err(|e| panic!("Failed to create db pool: {:?}", e))
        .unwrap();

    println!("Tosca REST-backend listening on https://localhost:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(web::scope("/api").configure(api::workspace::configure))
            .service(test)
    })
    .bind_rustls(("0.0.0.0", config.port), cfg)?
    .run()
    .await
}
