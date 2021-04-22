#![allow(dead_code)]

use crate::schema;
use crate::DbConnection;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use schema::{moderators, whitelists, workspaces};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: i32,
    pub creator: String,
    pub name: String,
    pub info: String,
    pub remote_workspace_id: Option<String>,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "workspaces"]
pub struct NewWorkspace<'a> {
    pub creator: &'a str,
    pub name: &'a str,
    pub info: &'a str,
    pub remote_workspace_id: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(Workspace)]
#[primary_key(workspace_id, user_id)]
pub struct Moderator {
    pub workspace_id: i32,
    pub user_id: String,
}

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(Workspace)]
#[primary_key(workspace_id, user_id)]
#[table_name = "whitelists"]
pub struct WhitelistEntry {
    pub workspace_id: i32,
    pub user_id: String,
}

/// Gets all stored workspaces. Does not fetch whitelist or moderators.
pub fn get_workspaces(con: &DbConnection) -> Result<Vec<Workspace>> {
    use schema::workspaces::dsl::*;
    workspaces
        .load::<Workspace>(con)
        .map_err(|x| anyhow!("Failed to load workspaces: {:?}", x))
}

/// Gets a workspace.
pub fn get_workspace_only(con: &DbConnection, workspace_id: i32) -> Result<Workspace> {
    workspaces::table
        .find(workspace_id)
        .first::<Workspace>(con)
        .map_err(|x| anyhow!("Failed to find workspace with id {}: {:?}", workspace_id, x))
}

/// Gets a workspace. Loads the workspace moderators and whitelist.
/// Returns a triple, `(Workspace, moderators: Vec<String>, whitelist: Vec<String>)`
pub fn get_workspace(
    con: &DbConnection,
    workspace_id: i32,
) -> Result<(Workspace, Vec<String>, Vec<String>)> {
    let wspace = get_workspace_only(con, workspace_id)?;

    let moderators = Moderator::belonging_to(&wspace)
        .select(moderators::user_id)
        .load(con)
        .map_err(|x| anyhow!("Failed to load moderators for workspace: {:?}", x))?;

    let whitelist = WhitelistEntry::belonging_to(&wspace)
        .select(whitelists::user_id)
        .load(con)
        .map_err(|x| anyhow!("Failed to load whitelist for workspace: {:?}", x))?;

    Ok((wspace, moderators, whitelist))
}

/// Stores a new workspace.
pub fn add_workspace<'a>(con: &DbConnection, wspace: &NewWorkspace<'a>) -> Result<Workspace> {
    use schema::workspaces::dsl::*;
    diesel::insert_into(workspaces)
        .values(wspace)
        .get_result::<Workspace>(con)
        .map_err(|e| anyhow!("Failed to insert new workspace: {:?}", e))
}

/// Inserts a moderator to a workspace.
pub fn add_moderator(con: &DbConnection, workspace_id: i32, new_mod: &str) -> Result<usize> {
    let new_mod = Moderator {
        workspace_id,
        user_id: new_mod.into(),
    };

    new_mod
        .insert_into(moderators::table)
        .execute(con)
        .map_err(|x| anyhow!("Failed to insert moderator: {:?}", x))
}

/// Deletes a moderator from a workspace.
pub fn delete_moderator(con: &DbConnection, workspace_id: i32, old_mod: &str) -> Result<usize> {
    let to_remove = Moderator {
        workspace_id,
        user_id: old_mod.into(),
    };

    diesel::delete(&to_remove)
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete moderator: {:?}", x))
}

/// Adds a user to the workspace whitelist.
pub fn add_whitelist_entry(con: &DbConnection, workspace_id: i32, user_id: &str) -> Result<usize> {
    let to_add = WhitelistEntry {
        workspace_id,
        user_id: user_id.into(),
    };

    to_add
        .insert_into(whitelists::table)
        .execute(con)
        .map_err(|x| anyhow!("Failed to insert user into whitelist: {:?}", x))
}

/// Deletes a user from a workspace whitelist.
pub fn delete_whitelist_entry(
    con: &DbConnection,
    workspace_id: i32,
    user_id: &str,
) -> Result<usize> {
    let to_remove = WhitelistEntry {
        workspace_id,
        user_id: user_id.into(),
    };

    diesel::delete(&to_remove)
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete whitelist entry: {:?}", x))
}

/// Updates the workspace with the given workspace id.
pub fn update_workspace<'a>(
    con: &DbConnection,
    workspace_id: i32,
    wspace: &NewWorkspace<'a>,
) -> Result<usize> {
    use schema::workspaces::dsl::*;
    let target = workspaces.filter(id.eq(workspace_id));

    diesel::update(target)
        .set(wspace)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update workspace: {:?}", e))
}
