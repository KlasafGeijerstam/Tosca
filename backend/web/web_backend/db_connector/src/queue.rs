use crate::schema;
use crate::workspace::{get_workspace_only, Workspace};
use crate::DbConnection;
use anyhow::{anyhow, Result};
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schema::{queue_slot_users, queue_slots, queues};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[belongs_to(Workspace)]
pub struct Queue {
    pub id: i32,
    pub workspace_id: i32,
    pub name: String,
    pub info: String,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "queues"]
pub struct NewQueue<'a> {
    pub workspace_id: i32,
    pub name: &'a str,
    pub info: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Debug, Deserialize, Serialize)]
#[belongs_to(Queue)]
pub struct QueueSlot {
    pub id: i32,
    pub queue_id: i32,
    #[serde(with = "ts_seconds")]
    pub start_time: NaiveDateTime,
    pub duration: i32,
    pub open_before: i32,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "queue_slots"]
pub struct NewQueueSlot {
    pub queue_id: i32,
    #[serde(with = "ts_seconds")]
    pub start_time: NaiveDateTime,
    pub duration: i32,
    pub open_before: i32,
}

#[derive(Identifiable, Queryable, Debug, Associations, Deserialize, Serialize)]
#[primary_key("queue_slot_user", "user_id")]
#[belongs_to(QueueSlot)]
pub struct QueueSlotUser {
    pub queue_slot_id: i32,
    pub user_id: String,
    pub message: String,
    pub moderator_message: String,
    #[serde(with = "ts_seconds")]
    pub q_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "queue_slot_users"]
pub struct NewQueueSlotUser<'a> {
    pub queue_slot_id: i32,
    pub user_id: &'a str,
    pub message: &'a str,
    pub moderator_message: &'a str,
    #[serde(with = "ts_seconds")]
    pub q_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "queue_slot_users"]
pub struct QueueSlotUserMessage<'a> {
    pub moderator_message: Option<&'a str>,
    pub message: Option<&'a str>,
}

/// Gets all queues for a workspace
pub fn get_queues(con: &DbConnection, workspace_id: i32) -> Result<Vec<Queue>> {
    let wspace = get_workspace_only(con, workspace_id)?;
    Queue::belonging_to(&wspace).load(con).map_err(|x| {
        anyhow!(
            "Failed to load queues for workspace {}: {:?}",
            workspace_id,
            x
        )
    })
}

/// Adds a queue for a workspace
pub fn add_queue<'a>(con: &DbConnection, new_queue: &NewQueue<'a>) -> Result<Queue> {
    use schema::queues::dsl::*;
    diesel::insert_into(queues)
        .values(new_queue)
        .get_result(con)
        .map_err(|e| anyhow!("Failed to insert new queue: {:?}", e))
}

/// Updates a queue
pub fn update_queue<'a>(con: &DbConnection, queue_id: i32, queue: &NewQueue<'a>) -> Result<usize> {
    use schema::queues::dsl::*;

    let target = queues.filter(id.eq(queue_id));

    diesel::update(target)
        .set(queue)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update queue: {:?}", e))
}

/// Gets a queue without its QueueSlots
pub fn get_queue_only(con: &DbConnection, queue_id: i32) -> Result<Queue> {
    queues::table
        .find(queue_id)
        .first(con)
        .map_err(|x| anyhow!("Failed to find queue with id {}: {:?}", queue_id, x))
}

/// Gets a Queue, along with its QueueSlots
pub fn get_queue(con: &DbConnection, queue_id: i32) -> Result<(Queue, Vec<QueueSlot>)> {
    let queue = get_queue_only(con, queue_id)?;
    let slots = QueueSlot::belonging_to(&queue)
        .select(queue_slots::all_columns)
        .get_results::<QueueSlot>(con)
        .map_err(|x| anyhow!("Failed to get queue slots for queue: {:?}", x))?;
    Ok((queue, slots))
}

pub fn delete_queue(con: &DbConnection, queue_id: i32) -> Result<usize> {
    use schema::queues::dsl::*;
    diesel::delete(queues)
        .filter(id.eq(queue_id))
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete queue: {:?}", x))
}

pub fn get_queue_slot(
    con: &DbConnection,
    queue_slot_id: i32,
) -> Result<(QueueSlot, Vec<QueueSlotUser>)> {
    let queue_slot: QueueSlot = queue_slots::table
        .find(queue_slot_id)
        .first(con)
        .map_err(|x| {
            anyhow!(
                "Failed to find queue_slot with id {}: {:?}",
                queue_slot_id,
                x
            )
        })?;

    let users = QueueSlotUser::belonging_to(&queue_slot)
        .load(con)
        .map_err(|x| anyhow!("Failed to get queue slot users: {:?}", x))?;

    Ok((queue_slot, users))
}

pub fn add_queue_slot(con: &DbConnection, new_queue_slot: &NewQueueSlot) -> Result<QueueSlot> {
    use schema::queue_slots::dsl::*;

    diesel::insert_into(queue_slots)
        .values(new_queue_slot)
        .get_result(con)
        .map_err(|x| anyhow!("Failed to insert new queue slot: {:?}", x))
}

pub fn update_queue_slot(
    con: &DbConnection,
    queue_slot_id: i32,
    queue_slot: &NewQueueSlot,
) -> Result<usize> {
    use schema::queue_slots::dsl::*;

    let target = queue_slots.filter(id.eq(queue_slot_id));

    diesel::update(target)
        .set(queue_slot)
        .execute(con)
        .map_err(|e| anyhow!("Failed to update queue slot: {:?}", e))
}

pub fn delete_queue_slot(con: &DbConnection, queue_slot_id: i32) -> Result<usize> {
    use schema::queue_slots::dsl::*;
    diesel::delete(queue_slots)
        .filter(id.eq(queue_slot_id))
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete queue_slot: {:?}", x))
}

pub fn add_queue_slot_user<'a>(con: &DbConnection, user: &NewQueueSlotUser<'a>) -> Result<usize> {
    user.insert_into(queue_slot_users::table)
        .execute(con)
        .map_err(|x| anyhow!("Failed to insert new queue slot user: {:?}", x))
}

pub fn update_queue_slot_user_message<'a>(
    con: &DbConnection,
    queue_slot_id: i32,
    user_id: &str,
    msg: &QueueSlotUserMessage<'a>,
) -> Result<usize> {
    diesel::update(queue_slot_users::table)
        .filter(queue_slot_users::queue_slot_id.eq(queue_slot_id))
        .filter(queue_slot_users::user_id.eq(user_id))
        .set(msg)
        .execute(con)
        .map_err(|x| anyhow!("Failed to update queue_slot_user_message: {:?}", x))
}

pub fn delete_queue_slot_user(
    con: &DbConnection,
    queue_slot_id: i32,
    user_id: &str,
) -> Result<usize> {
    diesel::delete(queue_slot_users::table)
        .filter(queue_slot_users::queue_slot_id.eq(queue_slot_id))
        .filter(queue_slot_users::user_id.eq(user_id))
        .execute(con)
        .map_err(|x| anyhow!("Failed to delete queue_slot_user: {:?}", x))
}
