use diesel::{
    pg::Pg, BoolExpressionMethods, ExpressionMethods, PgConnection, PgTextExpressionMethods,
    QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::{
    models::users::PublicUser,
    schema::users::BoxedQuery as UserQuery,
};

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
            Some(UserField::FirstName) => query.order(first_name.desc()),
            Some(UserField::LastName) => query.order(last_name.desc()),
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
            Some(UserField::FirstName) => query.order(first_name.asc()),
            Some(UserField::LastName) => query.order(last_name.asc()),
            Some(UserField::Email) => query.order(email.asc()),
            Some(UserField::CreatedAt) => query.order(created_at.asc()),
            Some(UserField::UpdatedAt) => query.order(updated_at.asc()),
            _ => query.order(id.asc()),
        },
    }
}

pub fn build<'a>(
    conn: &mut PgConnection,
    user: PublicUser,
    filter_search: &str,
    filter_status: Option<i16>,
    filter_role: Option<i16>,
) -> UserQuery<'a, diesel::pg::Pg> {
    use crate::schema::users::dsl as users;
    use crate::schema::workspace_members::dsl as workspace_members;

    // Declare the query (mutable)
    let mut query = users::users.into_boxed::<diesel::pg::Pg>();

    // Add the status filter
    if let Some(filter) = filter_status {
        query = query.filter(users::status.eq(filter))
    }

    // Add the role filter
    if let Some(filter) = filter_role {
        query = query.filter(users::role.eq(filter))
    }

    // Add the search filter
    if !filter_search.is_empty() {
        // Add excape characters for unsafe characters
        let safe_search = format!("{}", filter_search.replace('%', "\\%").replace('_', "\\_"));

        // Apply the search filter on the query
        query = query.filter(
            users::username
                .ilike(safe_search.clone())
                .or(users::email.ilike(safe_search.clone()))
                .or(users::first_name.ilike(safe_search.clone()))
                .or(users::last_name.ilike(safe_search.clone()))
                .or(users::phone.ilike(safe_search)),
        );
    }

    // Restrict to accessible users if non-admin
    if !user.is_admin() {
        // Find workspace_ids for current_user
        let workspace_ids: Vec<Uuid> = workspace_members::workspace_members
            .filter(workspace_members::member.eq(user.id))
            .select(workspace_members::workspace)
            .load(conn)
            .expect("Failed to fetch workspaces");

        // Find user_ids in those workspaces
        let accessible_user_ids: Vec<Uuid> = workspace_members::workspace_members
            .filter(workspace_members::workspace.eq_any(&workspace_ids))
            .select(workspace_members::member)
            .distinct()
            .load(conn)
            .expect("Failed to fetch accessible users");

        // Restrict query to those users
        query = query.filter(users::id.eq_any(accessible_user_ids));
    }

    query
}
