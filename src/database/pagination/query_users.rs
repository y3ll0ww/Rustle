use diesel::{pg::Pg, BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl};

use crate::schema::users::BoxedQuery as UserQuery;

use super::sort::{SortDirection, UserField};

pub fn sort<'a>(
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

pub fn build<'a>(
    filter_search: &str,
    filter_status: Option<i16>,
    filter_role: Option<i16>,
) -> UserQuery<'a, diesel::pg::Pg> {
    use crate::schema::users::dsl::*;

    let mut query = users.into_boxed::<diesel::pg::Pg>();

    if let Some(filter) = filter_status {
        query = query.filter(status.eq(filter))
    }

    if let Some(filter) = filter_role {
        query = query.filter(role.eq(filter))
    }

    // Add the following to the users table migration:
    // CREATE INDEX username_lower_idx ON users (LOWER(username));
    // CREATE INDEX email_lower_idx ON users (LOWER(email));
    // CREATE INDEX display_name_lower_idx ON users (LOWER(display_name));

    if !filter_search.is_empty() {
        let safe_search = filter_search.replace('%', "\\%").replace('_', "\\_");
        let q = format!("{safe_search}%");
        query = query.filter(
            username
                .ilike(q.clone())
                .or(email.ilike(q.clone()))
                .or(display_name.ilike(q)),
        );
    }

    query
}
