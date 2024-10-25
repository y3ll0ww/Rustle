use diesel::prelude::table;
use rocket_sync_db_pools::{database, diesel};

pub mod models;
pub mod schemas;

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);