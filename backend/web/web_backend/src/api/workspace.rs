use actix_web::{web, Error, Responder, HttpResponse, get, post, put, delete};
use super::DbPool;
use log::error;
use db_connector::workspace;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/workspaces")
        .service(get_workspaces)
        .service(post_workspaces));
}

#[get("/")]
async fn get_workspaces(db_pool: DbPool) -> Result<HttpResponse, Error> {
    let con = db_pool.get().expect("Failed to get database handle from pool");
    let workspaces = web::block(move || workspace::get_workspaces(&con))
        .await
        .map_err(|e| {
            error!("Failed to load workspaces: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(workspaces))
}

#[post("/")]
async fn post_workspaces(db_pool: DbPool) -> Result<HttpResponse, Error> {
    let con = db_pool.get().expect("Failed to get database handle from pool");
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

