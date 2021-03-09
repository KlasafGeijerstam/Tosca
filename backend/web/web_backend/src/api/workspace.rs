use actix_web::{web, web::Json, Error, Responder, HttpResponse, get, post, put, delete};
use super::DbPool;
use log::error;
use db_connector::workspace;
use serde::Deserialize;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/workspaces")
        .service(get_workspaces)
        .service(add_workspace));
}

use crate::user_provider::{UserData, AdminUser, SuperUser, NormalUser};

#[get("/")]
async fn get_workspaces(db_pool: DbPool, _user: UserData<NormalUser>) -> Result<HttpResponse, Error> {
    let con = db_pool.get().expect("Failed to get database handle from pool");
    let workspaces = web::block(move || workspace::get_workspaces(&con))
        .await
        .map_err(|e| {
            error!("Failed to load workspaces: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(workspaces))
}

#[derive(Deserialize)]
struct AddWorkspace {
    name: String,
}

#[post("/")]
async fn add_workspace(db_pool: DbPool, user: UserData<AdminUser>, wspace: Json<AddWorkspace>) -> Result<HttpResponse, Error> {
    let con = db_pool.get().expect("Failed to get database handle from pool");
    let workspace = workspace::NewWorkspace {
        creator: user.user_id,
        info: "".into(), 
        name: wspace.name.clone(),
    };
    
    let result = web::block(move || workspace::add_workspace(&con, &workspace))
        .await
        .map_err(|e| {
            error!("Failed to add workspace: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}

//* GET `/workspaces`
//* POST `/workspaces` [Super, Admin]
//* GET `/workspaces/{workspace_id}`
//* DELETE `workspaces/{workspace_id}` [Super, Creator]
//* POST `/workspaces/{workspace_id}/moderators` [Super, Creator, Moderator]
//* DELETE `/workspaces/{workspace_id}/moderators/{moderator_id}` [Super, Creator, Moderator]
//* DELETE `/workspaces/{workspace_id}/whitelist/` [Super, Creator, Moderator]
//* POST `/workspaces/{workspace_id}/whitelist` [Super, Creator, Moderator]
//* DELETE `/workspaces/{workspace_id}/whitelist/{user_id}` [Super, Creator, Moderator]

