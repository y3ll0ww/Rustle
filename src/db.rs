use rocket_sync_db_pools::{database, diesel};

#[database("rustle_db")]
pub struct Database(diesel::PgConnection);
