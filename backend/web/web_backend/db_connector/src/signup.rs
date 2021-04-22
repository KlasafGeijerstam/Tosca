use crate::schema;
use crate::workspace::{get_workspace_only, Workspace};
use crate::DbConnection;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schema::{signup_slot_users, signup_slots, signups};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[belongs_to(Workspace)]
pub struct Signup {
    pub id: i32,
    pub workspace_id: i32,
    pub max_slot_signup: i32,
    pub name: String,
    pub info: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "signups"]
pub struct NewSignup<'a> {
    pub workspace_id: i32,
    pub max_slot_signup: i32,
    pub name: &'a str,
    pub info: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[belongs_to(Signup)]
pub struct SignupSlot {
    pub id: i32,
    pub signup_id: i32,
    pub info: String,
    pub time: Option<NaiveDateTime>,
    pub max_users: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "signup_slots"]
pub struct NewSignupSlot<'a> {
    pub signup_id: i32,
    pub info: &'a str,
    pub time: Option<NaiveDateTime>,
    pub max_users: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "signup_slot_users"]
pub struct NewSignupSlotUser<'a> {
    pub signup_slot_id: i32,
    pub user_id: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[belongs_to(SignupSlot)]
#[primary_key("signup_slot_id", "user_id")]
pub struct SignupSlotUser {
    pub signup_slot_id: i32,
    pub user_id: String,
}

pub fn get_signups(con: &DbConnection, workspace_id: i32) -> Result<Vec<Signup>> {
    let wspace = get_workspace_only(con, workspace_id)?;
    Signup::belonging_to(&wspace).load(con).map_err(|x| {
        anyhow!(
            "Failed to load signups for workspace {}: {:?}",
            workspace_id,
            x
        )
    })
}

pub fn add_signup(con: &DbConnection, new_signup: &NewSignup) -> Result<Signup> {
    use schema::signups::dsl::*;
    diesel::insert_into(signups)
        .values(new_signup)
        .get_result(con)
        .map_err(|e| anyhow!("Failed to insrt new signup: {:?}", e))
}

pub fn update_signup(con: &DbConnection, signup_id: i32, signup: &NewSignup) -> Result<usize> {
    use schema::signups::dsl::*;
    let target = signups.filter(id.eq(signup_id));

    diesel::update(target)
        .set(signup)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update signup: {:?}", e))
}

pub fn get_signup_only(con: &DbConnection, signup_id: i32) -> Result<Signup> {
    use schema::signups::dsl::*;
    signups
        .find(signup_id)
        .first(con)
        .map_err(|e| anyhow!("Failed to find signup with id {}: {:?}", signup_id, e))
}

pub fn get_signup(con: &DbConnection, signup_id: i32) -> Result<(Signup, Vec<SignupSlot>)> {
    let signup = get_signup_only(con, signup_id)?;
    let slots = SignupSlot::belonging_to(&signup)
        .load::<SignupSlot>(con)
        .map_err(|x| anyhow!("Failed to get signup_slots: {:?}", x))?;

    Ok((signup, slots))
}

pub fn delete_signup(con: &DbConnection, signup_id: i32) -> Result<usize> {
    use schema::queue_slots::dsl::*;
    diesel::delete(queue_slots)
        .filter(id.eq(signup_id))
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete signup: {:?}", x))
}

pub fn get_signup_slot_only(con: &DbConnection, signup_slot_id: i32) -> Result<SignupSlot> {
    use schema::signup_slots::dsl::*;
    signup_slots
        .find(signup_slot_id)
        .first(con)
        .map_err(|e| anyhow!("Failed to find signup_slot: {:?}", e))
}

pub fn get_signup_slot(
    con: &DbConnection,
    signup_slot_id: i32,
) -> Result<(SignupSlot, Vec<SignupSlotUser>)> {
    let slot = get_signup_slot_only(con, signup_slot_id)?;
    let users = SignupSlotUser::belonging_to(&slot)
        .load(con)
        .map_err(|e| anyhow!("Failed to get signup_slot_users: {:?}", e))?;

    Ok((slot, users))
}

pub fn add_signup_slot(con: &DbConnection, signup_slot: &NewSignupSlot) -> Result<SignupSlot> {
    use schema::signup_slots::dsl::*;

    diesel::insert_into(signup_slots)
        .values(signup_slot)
        .get_result(con)
        .map_err(|e| anyhow!("Failed to add signup_slot: {:?}", e))
}

pub fn update_signup_slot(
    con: &DbConnection,
    signup_slot_id: i32,
    signup_slot: &NewSignupSlot,
) -> Result<usize> {
    let target = get_signup_slot_only(con, signup_slot_id)?;
    diesel::update(&target)
        .set(signup_slot)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update signup_slot: {:?}", e))
}

pub fn delete_signup_slot(con: &DbConnection, signup_slot_id: i32) -> Result<usize> {
    use schema::signup_slots::dsl::*;
    diesel::delete(signup_slots)
        .filter(id.eq(signup_slot_id))
        .execute(con)
        .map_err(|e| anyhow!("Failed to delete signup_slot: {:?}", e))
}

pub fn add_signup_slot_user(
    con: &DbConnection,
    user: &NewSignupSlotUser,
) -> Result<SignupSlotUser> {
    use schema::signup_slot_users::dsl::*;
    diesel::insert_into(signup_slot_users)
        .values(user)
        .get_result(con)
        .map_err(|e| anyhow!("Failed to add signup_slot_user: {:?}", e))
}

pub fn delete_signup_slot_user(con: &DbConnection, slot_id: i32, u_id: &str) -> Result<usize> {
    use schema::signup_slot_users::dsl::*;
    diesel::delete(signup_slot_users)
        .filter(signup_slot_id.eq(slot_id))
        .filter(user_id.eq(u_id))
        .execute(con)
        .map_err(|e| anyhow!("Failed to delete signup_slot_user: {:?}", e))
}
