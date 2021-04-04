use super::DbPool;
use actix_web::{get, post, web, web::Json, Error, HttpResponse};
use db_connector::{queue::Queue, workspace, workspace::Workspace};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RestWorkspace {
    #[serde(flatten)]
    workspace: Workspace,
    queues: Vec<Queue>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workspaces")
            .service(get_workspaces)
            .service(add_workspace),
    );
}

use crate::user_provider::{AdminUser, NormalUser, UserData};

#[get("")]
async fn get_workspaces(
    db_pool: DbPool,
    _user: UserData<NormalUser>,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");
    let workspaces = web::block(move || {
        workspace::get_workspaces(&con)?
            .drain(..)
            .map(|workspace| {
                Ok(RestWorkspace {
                    workspace,
                    queues: Vec::new(),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    })
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
    info: String,
}

#[post("")]
async fn add_workspace(
    db_pool: DbPool,
    user: UserData<AdminUser>,
    wspace: Json<AddWorkspace>,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");

    let result = web::block(move || {
        let workspace = workspace::NewWorkspace {
            creator: &user.user_id,
            info: &wspace.info,
            name: &wspace.name,
        };
        workspace::add_workspace(&con, &workspace)
    })
    .await
    .map_err(|e| {
        error!("Failed to add workspace: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().json(result))
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
