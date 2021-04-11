use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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

    println!("Listening on port: {}", args.port);
    println!("Redirecting to: {}", args.client_endpoint);

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

async fn read_index(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX)
}

const INDEX: &'static str = r#"
<html>
  <head>
    <meta charset="UTF-8">
  </head>
  <body> 
    <div id="login">
      <form action="do_login">
        <div>
          <label>Username:</label>
          <input type="text" name="user_id">
        </div>
        <button>Login!</button>
      </form>
    </div>
    <style>
      #login {
        display: flex;
        justify-content: center;
        height: 100%;
      }
      
      input {
        margin: 0.5em;
      }

      form {
        align-self: center;
        border: 1px solid black;
        padding: 5em;
        border-radius: 1em;
        box-shadow: 5px 5px lightgray;
        display: flex;
        flex-direction: column;
      }
    </style>
  <body>
</html>
"#;
