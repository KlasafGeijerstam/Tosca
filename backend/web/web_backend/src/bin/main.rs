use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Result};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use db_connector::create_db_pool_env;
use web_backend::user_provider::{UserData, UserProvider, SuperUser, AdminUser, NormalUser };
use web_backend::login_provider::LoginProvider;
use web_backend::api;

#[derive(StructOpt)]
#[structopt(name = "Tosca REST-backend")]
struct Arguments {
    /// The config file (in TOML format)
    config_file: String,

    /// Toggles debug mode.
    #[structopt(long)]
    debug: bool,
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

#[get("/super")]
async fn super_user(user: UserData<SuperUser>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {}", user.first_name))
}

#[get("/normal")]
async fn normal_user(user: UserData<NormalUser>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {}", user.first_name))
}

#[get("/admin")]
async fn admin_user(user: UserData<AdminUser>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {}", user.first_name))
}


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

    let user_provider = web::Data::new(UserProvider::new("REPLACEME"));
    let login_provider = web::Data::new(LoginProvider::new("REPLACEME"));

    println!(
        "Tosca REST-backend listening on https://localhost:{}",
        config.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(middleware::normalize::TrailingSlash::Trim))
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .app_data(user_provider.clone())
            .app_data(login_provider.clone())
            .service(web::scope("/api").configure(api::workspace::configure))
            .service(super_user)
            .service(normal_user)
            .service(admin_user)
    })
    .bind_rustls(("0.0.0.0", config.port), cfg)?
    .run()
    .await
}
