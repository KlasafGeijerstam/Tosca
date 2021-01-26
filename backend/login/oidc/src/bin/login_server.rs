use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

use openidtest::{Config, OpenIDCallbackInfo, OpenIDProvider};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Tosca OpenID Connect provider")]
struct Arguments {
    /// The config file (in TOML format)
    config_file: String,
}

struct AppData {
    redirect_uri: String,
    client_endpoint: String,
    provider: OpenIDProvider,
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

#[get("/login")]
async fn login(data: web::Data<AppData>) -> impl Responder {
    let url = data.provider.get_login_link(&data.redirect_uri).await;
    HttpResponse::TemporaryRedirect()
        .header("Location", url)
        .finish()
}

#[get("/token")]
async fn token(token: web::Json<Token>, data: web::Data<AppData>) -> impl Responder {
    match data.provider.validate_session_token(&token.token).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(error) => HttpResponse::NotAcceptable().body(error.to_string()),
    }
}

#[get("/after_oidc")]
async fn after_oidc(
    resp: web::Query<OpenIDCallbackInfo>,
    data: web::Data<AppData>,
) -> impl Responder {
    let claims = data
        .provider
        .verify_callback(&resp, &data.redirect_uri)
        .await;
    let mut builder = HttpResponse::TemporaryRedirect();

    let params = match claims {
        Ok((u_token, expires_in)) => [("token", u_token), ("exp", expires_in.to_string())],
        Err(error) => [("error", error.to_string()), ("", "".into())],
    };

    let client = Client::new();
    let reqwest = client
        .get(&data.client_endpoint)
        .query(&params)
        .build()
        .unwrap();

    println!("Redirecting to {}", reqwest.url());
    builder
        .header("Location", format!("{}", reqwest.url()))
        .finish()
}

#[post("/logout")]
async fn logout(data: web::Data<AppData>, user_token: web::Json<Token>) -> impl Responder {
    data.provider.logout(&user_token.token).await;

    HttpResponse::Ok().finish()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();

    let oidc_cfg = Config::from_config_file(&args.config_file)
        .await
        .unwrap();

    let cfg = load_ssl_keys(&oidc_cfg);

    let provider = web::Data::new(AppData {
        provider: OpenIDProvider::new(oidc_cfg.provider).await.unwrap(),
        redirect_uri: format!("{}/after_oidc", oidc_cfg.public_host),
        client_endpoint: oidc_cfg.client_endpoint,
    });

    HttpServer::new(move || {
        App::new()
            .service(login)
            .service(logout)
            .service(token)
            .service(after_oidc)
            .app_data(provider.clone())
    })
    .bind_rustls(("0.0.0.0", oidc_cfg.port), cfg)?
    .run()
    .await
}
