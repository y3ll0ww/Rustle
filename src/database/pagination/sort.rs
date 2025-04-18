use diesel::{pg::Pg, ExpressionMethods, QueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::users::BoxedQuery as UserQuery;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

pub trait SortField {}

impl SortField for UserField {}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserField {
    Role,
    Status,
    Username,
    DisplayName,
    Email,
    CreatedAt,
    UpdatedAt,
}

pub fn sort_users<'a>(
    query: UserQuery<'a, Pg>,
    sort_by: &Option<UserField>,
    sort_dir: &Option<SortDirection>,
) -> UserQuery<'a, Pg> {
    use crate::schema::users::dsl::*;

    match sort_dir {
        Some(SortDirection::Desc) => match sort_by {
            Some(UserField::Role) => query.order(role.desc()),
            Some(UserField::Status) => query.order(status.desc()),
            Some(UserField::Username) => query.order(username.desc()),
            Some(UserField::DisplayName) => query.order(display_name.desc()),
            Some(UserField::Email) => query.order(email.desc()),
            Some(UserField::CreatedAt) => query.order(created_at.desc()),
            Some(UserField::UpdatedAt) => query.order(updated_at.desc()),
            _ => query.order(id.desc()),
        },
        // Ascending by default
        _ => match sort_by {
            Some(UserField::Role) => query.order(role.asc()),
            Some(UserField::Status) => query.order(status.asc()),
            Some(UserField::Username) => query.order(username.asc()),
            Some(UserField::DisplayName) => query.order(display_name.asc()),
            Some(UserField::Email) => query.order(email.asc()),
            Some(UserField::CreatedAt) => query.order(created_at.asc()),
            Some(UserField::UpdatedAt) => query.order(updated_at.asc()),
            _ => query.order(id.asc()),
        },
    }
}
