use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer};

#[derive(Deserialize)]
struct Config {
    users: HashMap<String, User>,
}

#[derive(Deserialize, Serialize, Clone)]
struct User {
    name: String,
    permissions: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct UserWithID {
    uid: String,
    name: String,
    permissions: u8,
}

#[derive(Deserialize, Serialize, Clone)]
struct Group {
    gid: String,
    users: Vec<String>,
}

const FILE: &'static str = "res/dev_foreign.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(FILE);

    let data =
        String::from_utf8(fs::read(file_path)?).expect("Config file does not contain valid utf8");

    let config: Config = toml::from_str(&data)?;

    let users: HashMap<_, _> = config
        .users
        .iter()
        .map(|(k, u)| {
            (
                k.clone(),
                UserWithID {
                    uid: k.clone(),
                    name: u.name.clone(),
                    permissions: u.permissions,
                },
            )
        })
        .collect();

    println!("Loaded {} users", config.users.len());
    for user in users.values() {
        println!("{:?}", user);
    }
    println!();

    let users: Users = web::Data::new(users);

    let groups: Groups = web::Data::new(HashMap::new());

    println!("Foreign dev-server listening on port 8000");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(users.clone())
            .app_data(groups.clone())
            .service(
                web::scope("/api")
                    .service(validate_token)
                    .service(destroy_token)
                    .service(get_user_data)
                    .service(get_group_users),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

type Groups = web::Data<HashMap<String, Group>>;
type Users = web::Data<HashMap<String, UserWithID>>;
type Token = web::Path<String>;
type UserID = Token;
type GroupID = Token;

//* ValidateToken(Token) -> UserData
//* DestroyToken(Token) -> ()
//* GetUser(Token, UserID) -> UserData
//* GetGroupUsers(Token, GroupID) -> [UserID]

fn check_token(token: &Token, users: &Users) -> bool {
    users.contains_key(&**token)
}

#[get("/{token}/validate")]
async fn validate_token(token: Token, users: Users) -> HttpResponse {
    if let Some(user) = users.get(&*token) {
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/{token}/destroy")]
async fn destroy_token(_token: Token) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/{token}/users/{userid}")]
async fn get_user_data(token: Token, uid: UserID, users: Users) -> HttpResponse {
    if !check_token(&token, &users) {
        return HttpResponse::Unauthorized().finish();
    }

    if let Some(user) = users.get(&*uid) {
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/{token}/group/{group}/")]
async fn get_group_users(token: Token, gid: GroupID, groups: Groups, users: Users) -> HttpResponse {
    if !check_token(&token, &users) {
        return HttpResponse::Unauthorized().finish();
    }

    if let Some(group) = groups.get(&*gid) {
        HttpResponse::Ok().json(group.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}
