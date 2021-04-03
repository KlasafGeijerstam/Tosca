use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Tosca development login provider")]
struct Arguments {
    /// The port to listen on
    port: u16,

    /// The URI to redirect logged in clients to
    client_endpoint: String,
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct LoginData {
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    sub: String,
    exp: u64,
}

#[get("/token")]
async fn token(token: web::Json<Token>) -> impl Responder {
    if !token.token.starts_with("token_") {
        HttpResponse::Unauthorized().body("Invalid token")
    } else {
        let uid = token.token.strip_prefix("token_").unwrap();
        HttpResponse::Ok().json(UserData {
            sub: uid.into(),
            exp: 9999,
        })
    }
}

#[get("/do_login")]
async fn do_login(resp: web::Query<LoginData>, cfg: web::Data<Arguments>) -> impl Responder {
    let url = format!(
        "{}?token=token_{}&exp=99999",
        cfg.client_endpoint, resp.user_id
    );
    println!("Redirecting to {}", url);

    HttpResponse::TemporaryRedirect()
        .header("Location", url)
        .finish()
}

#[post("/logout")]
async fn logout(user_token: web::Json<Token>) -> impl Responder {
    println!("Logging out: {}", user_token.token);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();
    let port = args.port;
    let data = web::Data::new(args);

    pretty_env_logger::init();

    HttpServer::new(move || {
        App::new()
            .route("/login", web::get().to(read_index))
            .wrap(actix_web::middleware::Logger::default())
            .service(do_login)
            .service(logout)
            .service(token)
            .app_data(data.clone())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn read_index(_: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}
