use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

use crate::models::users::PublicUser;

pub mod projects;
pub mod users;
pub mod workspaces;

#[derive(Deserialize, Queryable, Serialize)]
pub struct MemberInfo {
    pub user: PublicUser,
    pub role: i16,
}
