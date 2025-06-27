use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

pub trait SortField {}

impl SortField for UserField {}
impl SortField for ProjectField {}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserField {
    Role,
    Status,
    Username,
    FirstName,
    LastName,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectField {
    Name,
    MemberCount,
    CreatedAt,
    UpdatedAt,
}
