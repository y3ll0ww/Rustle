pub mod pagination;
pub mod users;
pub mod workspaces;

use rocket_sync_db_pools::{database, diesel};

#[database("rustle_db")]
pub struct Db(diesel::PgConnection);
