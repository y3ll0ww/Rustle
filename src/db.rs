use rocket_sync_db_pools::{database, diesel};

pub mod api;

#[database("sqlite_db")]
pub struct Db(diesel::SqliteConnection);
