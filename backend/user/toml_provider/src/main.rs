use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;
use user_format::*;
use actix_web::{get, web, web::Path, App, HttpResponse, HttpServer};


#[derive(StructOpt)]
struct Arguments {
    /// The TOML file containing the user database to serve.
    user_database: String,

    /// The port to listen on.
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::from_args();

    let data = fs::read_to_string(&args.user_database)?;

    let mut config: Config = toml::from_str(&data)?;

    let workspaces = config.workspaces.clone();
    let users: HashMap<_, _> = config
        .users
        .drain()
        .map(|(k, u)| {
            let user_workspaces = workspaces
                .iter()
                .filter(|(_, workspace)| workspace.users.contains(&k))
                .map(|(gid, _)| gid.clone())
                .collect();

            (
                k.clone(),
                UserWithID {
                    user_id: k,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    user_level: u.user_level,
                    workspaces: user_workspaces,
                },
            )
        })
        .collect();

    println!("Loaded {} users", users.len());
    for user in users.values() {
        println!("{:?}", user);
    }
    println!();

    let users: Users = web::Data::new(users);

    let workspaces: Workspaces = web::Data::new(config.workspaces);

    println!("TOML user provider listening on port {}", args.port);

    pretty_env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(users.clone())
            .app_data(workspaces.clone())
            .service(
                web::scope("/")
                    .service(get_user)
                    .service(get_workspace)
                    .service(get_workspaces),
            )
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}

type Workspaces = web::Data<HashMap<String, Workspace>>;
type Users = web::Data<HashMap<String, UserWithID>>;

#[get("/users/{user_id}")]
async fn get_user(Path(user_id): Path<String>, users: Users) -> HttpResponse {
    if let Some(user) = users.get(&user_id) {
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/workspaces")]
async fn get_workspaces(workspaces: Workspaces) -> HttpResponse {
    HttpResponse::Ok().json((workspaces.keys().collect::<Vec<_>>()).clone())
}

#[get("/workspaces/{workspace_id}")]
async fn get_workspace(Path(workspace_id): Path<String>, workspaces: Workspaces) -> HttpResponse {
    if let Some(workspace) = workspaces.get(&workspace_id) {
        HttpResponse::Ok().json(workspace.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}
