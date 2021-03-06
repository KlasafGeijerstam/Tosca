use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Result};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use structopt::StructOpt;

use db_connector::create_db_pool;
use web_backend::api;
use web_backend::login_provider::LoginProvider;
use web_backend::user_provider::{AdminUser, NormalUser, SuperUser, UserProvider};

use actix_cors::Cors;

#[derive(StructOpt)]
#[structopt(name = "Tosca REST-backend")]
struct Arguments {
    /// The config file (in TOML format)
    config_file: String,

    /// Database-url Overrides database configuration.
    /// Example: `postgres://tosca_user:password@localhost/tosca_database`
    #[structopt(long)]
    database: Option<String>,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    database: String,
    password: String,
}

impl DatabaseConfig {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Deserialize)]
struct Config {
    port: u16,
    certificate_file: String,
    key_file: String,
    login_provider: String,
    user_provider: String,

    database: DatabaseConfig,
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

#[get("/ping")]
async fn ping() -> impl Responder {
    "Pong!"
}

#[get("/test/super")]
async fn super_user(user: SuperUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {:?}", user))
}

#[get("/test/normal")]
async fn normal_user(user: NormalUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {:?}", user))
}

#[get("/test/admin")]
async fn admin_user(user: AdminUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello! {:?}", user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();

    let config = parse_config(&args.config_file).unwrap();
    let cfg = load_ssl_keys(&config);

    pretty_env_logger::init();

    let db_url = if let Some(url) = args.database {
        url
    } else {
        config.database.to_url()
    };

    let db_pool = create_db_pool(&db_url)
        .map_err(|e| panic!("Failed to create db pool: {:?}", e))
        .unwrap();

    let user_provider = web::Data::new(UserProvider::new(&config.user_provider));
    let login_provider = web::Data::new(LoginProvider::new(&config.login_provider));

    println!(
        "Tosca REST-backend listening on https://localhost:{}",
        config.port
    );

    println!("Using login provider: {}", config.login_provider);
    println!("Using user provider: {}", config.user_provider);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .app_data(user_provider.clone())
            .app_data(login_provider.clone())
            .service(
                web::scope("/api")
                    .configure(api::queue::configure)
                    .configure(api::workspace::configure),
            )
            .service(super_user)
            .service(normal_user)
            .service(admin_user)
            .service(ping)
    })
    .bind_rustls(("0.0.0.0", config.port), cfg)?
    .run()
    .await
}
