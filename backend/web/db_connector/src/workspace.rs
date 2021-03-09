#![allow(dead_code)]

use crate::schema;
use crate::DbConnection;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use schema::{moderator, whitelist, workspace};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Debug, Deserialize, Serialize)]
#[primary_key("workspace_id")]
#[table_name = "workspace"]
pub struct Workspace {
    pub workspace_id: i32,
    pub creator: String,
    pub name: String,
    pub info: String,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "workspace"]
pub struct NewWorkspace {
    pub creator: String,
    pub name: String,
    pub info: String,
}

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(Workspace, foreign_key = "workspace_id")]
#[primary_key(workspace_id, user_id)]
#[table_name = "moderator"]
pub struct Moderator {
    pub workspace_id: i32,
    pub user_id: String,
}

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(Workspace, foreign_key = "workspace_id")]
#[primary_key(workspace_id, user_id)]
#[table_name = "whitelist"]
pub struct WhitelistEntry {
    pub workspace_id: i32,
    pub user_id: String,
}

/// Gets all stored workspaces. Does not fetch whitelist or moderators.
pub fn get_workspaces(con: &DbConnection) -> Result<Vec<Workspace>> {
    use schema::workspace::dsl::*;
    workspace
        .load::<Workspace>(con)
        .map_err(|x| anyhow!("Failed to load workspaces: {:?}", x))
}

/// Gets a workspace. Loads the workspace moderators and whitelist.
/// Returns a triple, `(Workspace, moderators: Vec<String>, whitelist: Vec<String>)`
pub fn get_workspace(
    con: &DbConnection,
    wspace_id: i32,
) -> Result<(Workspace, Vec<String>, Vec<String>)> {
    let wspace = workspace::table
        .find(wspace_id)
        .first::<Workspace>(con)
        .map_err(|x| anyhow!("Failed to find workspace with id {}: {:?}", wspace_id, x))?;

    let moderators = moderator::table
        .filter(moderator::workspace_id.eq(wspace.workspace_id))
        .select(moderator::user_id)
        .load(con)
        .map_err(|x| anyhow!("Failed to load moderators for workspace: {:?}", x))?;

    let whitelist = whitelist::table
        .filter(whitelist::workspace_id.eq(wspace.workspace_id))
        .select(whitelist::user_id)
        .load(con)
        .map_err(|x| anyhow!("Failed to load whitelist for workspace: {:?}", x))?;

    Ok((wspace, moderators, whitelist))
}

/// Stores a new workspace.
pub fn add_workspace(con: &DbConnection, wspace: &NewWorkspace) -> Result<usize> {
    use schema::workspace::dsl::*;
    diesel::insert_into(workspace)
        .values(wspace)
        .execute(con)
        .map_err(|e| anyhow!("Failed to insert new workspace: {:?}", e))
}

/// Inserts a moderator to a workspace.
pub fn add_moderator(con: &DbConnection, workspace_id: i32, new_mod: &str) -> Result<usize> {
    let new_mod = Moderator {
        workspace_id,
        user_id: new_mod.into(),
    };

    new_mod
        .insert_into(moderator::table)
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
        .insert_into(whitelist::table)
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
pub fn update_workspace(con: &DbConnection, wspace_id: i32, wspace: &NewWorkspace) -> Result<usize> {
    use schema::workspace::dsl::*;
    let target = workspace.filter(workspace_id.eq(wspace_id));

    diesel::update(target)
        .set(wspace)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update workspace: {:?}", e))
}
