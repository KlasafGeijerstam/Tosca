use super::{DbPool, UserProvider};
use crate::user_provider::user_field::UserField;
use crate::user_provider::{AdminUser, NormalUser};
use actix_web::{get, post, web, web::Json, Error, HttpResponse};
use db_connector::{
    queue::get_queues, queue::Queue, workspace, workspace::Workspace as DBWorkspace,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RestWorkspace {
    #[serde(flatten)]
    workspace: DBWorkspace,
    creator: UserField,
    queues: Vec<Queue>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workspaces")
            .service(get_workspaces)
            .service(add_workspace),
    );
}

#[get("")]
async fn get_workspaces(
    db_pool: DbPool,
    _user: NormalUser,
    user_provider: UserProvider,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");
    let mut workspaces = web::block(move || {
        workspace::get_workspaces(&con)?
            .drain(..)
            .map(|workspace| {
                Ok(RestWorkspace {
                    queues: get_queues(&con, workspace.id)?,
                    creator: workspace.creator_user_id.clone().into(),
                    workspace,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    })
    .await
    .map_err(|e| {
        error!("Failed to load workspaces: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    for workspace in workspaces.iter_mut() {
        workspace.creator.lookup(&user_provider).await?;
    }

    Ok(HttpResponse::Ok().json(workspaces))
}

#[derive(Deserialize)]
struct AddWorkspace {
    name: String,
    info: String,
    remote_workspace_id: Option<String>,
}

#[post("")]
async fn add_workspace(
    db_pool: DbPool,
    user: AdminUser,
    wspace: Json<AddWorkspace>,
) -> Result<HttpResponse, Error> {
    let con = db_pool
        .get()
        .expect("Failed to get database handle from pool");

    let result = web::block(move || {
        let workspace = workspace::NewWorkspace {
            creator_user_id: &user.user_id(),
            info: &wspace.info,
            name: &wspace.name,
            remote_workspace_id: wspace.remote_workspace_id.as_deref()
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
