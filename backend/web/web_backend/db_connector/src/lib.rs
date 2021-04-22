#[macro_use]
extern crate diesel;
mod schema;

pub mod queue;
pub mod signup;
pub mod workspace;

use anyhow::{anyhow, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Creates a new r2d2 database pool from a connection string
/// Connection string format: `postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE`
pub fn create_db_pool(database_url: &str) -> Result<DbPool> {
    r2d2::Pool::builder()
        .build(ConnectionManager::new(database_url))
        .map_err(|e| anyhow!("Failed to create database pool: {:?}", e))
}

/// Creates a new r2d2 database pool configured from environment variables.
/// The `DATABASE_URL` environment variable must be set.
/// DATABASE_URL format: `postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE`
pub fn create_db_pool_env() -> Result<DbPool> {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").map_err(|_| anyhow!("DATABASE_URL must be set"))?;

    create_db_pool(&db_url)
}
