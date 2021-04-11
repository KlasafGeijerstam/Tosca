use super::DbPool;
use actix_web::{get, post, web, web::Json, Error, HttpResponse};
use db_connector::{queue, queue::Queue};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RestQueue {
    #[serde(flatten)]
    queue: Queue,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/queue").service(get_queue).service(add_queue));
}

use crate::user_provider::{AdminUser, NormalUser};

#[derive(Deserialize)]
struct QueueParam {
    q_id: i32,
}

#[get("/{q_id}")]
async fn get_queue(
    db_pool: DbPool,
    _user: NormalUser,
    param: web::Path<QueueParam>,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");
    let queue = web::block(move || queue::get_queue(&con, param.q_id))
        .await
        .map_err(|e| {
            error!("Failed to load queue: {:?}", e);
            HttpResponse::NotFound().finish()
        })?;

    Ok(HttpResponse::Ok().json(queue))
}

#[derive(Deserialize)]
struct AddQueue {
    workspace_id: i32,
    name: String,
    info: String,
}

#[post("")]
async fn add_queue(
    db_pool: DbPool,
    _: AdminUser,
    add_queue: Json<AddQueue>,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");

    let result = web::block(move || {
        let queue = queue::NewQueue {
            workspace_id: add_queue.workspace_id,
            name: &add_queue.name,
            info: &add_queue.info,
        };
        queue::add_queue(&con, &queue)
    })
    .await
    .map_err(|e| {
        error!("Failed to add queue: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(result))
}
