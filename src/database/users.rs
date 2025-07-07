use std::collections::HashSet;

use diesel::{
    result::Error as DieselError, Connection, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    database::{pagination::queries::meta::PaginationMetaData, Db},
    models::users::{PublicUser, User, UserStatus, UserUpdate},
    schema::{users, workspace_members},
};

use super::pagination::{
    queries::users as query_users, records::PaginatedRecords, request::PaginationRequest,
    sort::UserField,
};

pub async fn get_all_public_users(db: &Db) -> Result<Vec<PublicUser>, Error<Null>> {
    let users: Vec<PublicUser> = db
        .run(move |conn| users::table.get_results::<User>(conn))
        .await
        .map_err(ApiResponse::from_error)?
        .iter()
        .map(PublicUser::from)
        .collect();

    Ok(users)
}

pub async fn get_all_public_users_from_workspaces(
    db: &Db,
    user: Uuid,
) -> Result<Vec<PublicUser>, Error<Null>> {
    let users: Vec<PublicUser> = db
        .run(move |conn| {
            users::table
                .inner_join(workspace_members::table.on(workspace_members::member.eq(users::id)))
                .filter(workspace_members::member.eq(user))
                .select((
                    users::id,
                    users::username,
                    users::first_name,
                    users::last_name,
                    users::email,
                    users::phone,
                    users::role,
                    users::status,
                    users::job_title,
                    users::password,
                    users::bio,
                    users::avatar_url,
                    users::created_at,
                    users::updated_at,
                ))
                .get_results::<User>(conn)
        })
        .await
        .map_err(ApiResponse::from_error)?
        .iter()
        .map(PublicUser::from)
        .collect();

    Ok(users)
}

/// Returns user information paginated. Admin will return all users, where other users will only
/// return users with whom they share a workspace with.
pub async fn get_users_paginated(
    db: &Db,
    user: PublicUser,
    status: Option<i16>,
    role: Option<i16>,
    params: Json<PaginationRequest<UserField>>,
) -> Result<PaginatedRecords<PublicUser>, Error<Null>> {
    // Extract the pagination request
    let params = params.into_inner();

    let (meta, users) = db
        .run(move |conn| {
            // Define the search string
            let search = params.search.as_deref().unwrap_or_default();

            // Build the query as COUNT to get the total
            let total = query_users::build(conn, user.clone(), search, status, role)
                .count()
                .get_result::<i64>(conn)?;

            // Calculate the pagination meta data
            let meta = PaginationMetaData::new(total, &params);

            // Build the query again for LOAD and apply filtering
            let mut query = query_users::build(conn, user, search, status, role);

            // Apply sorting to the query
            query = query_users::sort(query, &params.sort_by, &params.sort_dir);

            // Add the offset and limit and run the query
            let users: Vec<PublicUser> = query
                .offset(meta.record_offset)
                .limit(meta.page_limit)
                .load::<User>(conn)
                .map(|users| users.iter().map(PublicUser::from).collect())?;

            Ok((meta, users))
        })
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(
        PaginatedRecords::<PublicUser>::new(meta, users),
    )
}

pub async fn get_user_by_id(db: &Db, id: Uuid) -> Result<User, Error<Null>> {
    db.run(move |conn| users::table.filter(users::id.eq(id)).first::<User>(conn))
        .await
        .map_err(ApiResponse::from_error)
}

pub async fn get_user_by_username(db: &Db, username: &str) -> Result<User, Error<Null>> {
    let username = username.to_string();

    db.run(move |conn| {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn get_username_duplicates(
    db: &Db,
    base_usernames: &HashSet<String>,
) -> Result<HashSet<String>, Error<Null>> {
    // Pattern for matching usernames; exact and numbered variants:
    // ^(john_doe|jane_doe|john_smith|jane_smith)(_[0-9]+)?$
    let regex_pattern = format!(
        "^({})(_[0-9]+)?$",
        base_usernames
            .iter()
            .map(|name| regex::escape(name))
            .collect::<Vec<_>>()
            .join("|")
    );

    // Get all existing similar usernames from the database
    db.run({
        // Get all existing usernames from the database using the regex pattern, then collect
        // their usernames into a HashSet
        move |conn| {
            diesel::sql_query("SELECT * FROM users WHERE username ~ $1")
                .bind::<diesel::sql_types::Text, _>(&regex_pattern)
                .load::<User>(conn)
                .map(|users| users.into_iter().map(|u| u.username).collect())
        }
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn update_user_information(
    db: &Db,
    id: Uuid,
    update: UserUpdate,
) -> Result<PublicUser, Error<Null>> {
    db.run(move |conn| {
        diesel::update(users::table.filter(users::id.eq(id)))
            .set(update)
            .get_result::<User>(conn)
            .map_err(ApiResponse::from_error)
            .map(|user| PublicUser::from(&user))
    })
    .await
}

pub async fn update_user_status(db: &Db, id: Uuid, status: i16) -> Result<PublicUser, Error<Null>> {
    db.run(move |conn| {
        diesel::update(users::table.filter(users::id.eq(id)))
            .set(users::status.eq(status))
            .get_result::<User>(conn)
            .map_err(ApiResponse::from_error)
            .map(|user| PublicUser::from(&user))
    })
    .await
}

pub async fn update_user_role(db: &Db, id: Uuid, role: i16) -> Result<PublicUser, Error<Null>> {
    db.run(move |conn| {
        diesel::update(users::table.filter(users::id.eq(id)))
            .set(users::role.eq(role))
            .get_result::<User>(conn)
            .map_err(ApiResponse::from_error)
            .map(|user| PublicUser::from(&user))
    })
    .await
}

pub async fn create_transaction_bulk_invitation(
    new_users: &[User],
    db: &Db,
) -> Result<usize, Error<Null>> {
    // Insert into database with a single transaction
    db.run({
        // Clone the new_users vector to move into the closure
        let insert_users = new_users.to_owned();

        // Move the database connection into the closure
        move |conn| {
            // Insert all users in one transaction; if any error occurs, rollback
            conn.build_transaction().read_write().run(|conn| {
                diesel::insert_into(users::dsl::users)
                    .values(&insert_users)
                    .execute(conn)
            })
        }
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn set_user_password(
    db: &Db,
    id: Uuid,
    password_hash: String,
) -> Result<usize, Error<Null>> {
    db.run(move |conn| {
        diesel::update(users::table.filter(users::id.eq(&id)))
            .set((
                users::password.eq(&password_hash),
                users::status.eq(i16::from(UserStatus::Active)),
            ))
            .execute(conn)
            .map_err(ApiResponse::from_error)
    })
    .await
}

pub async fn delete_user_by_id(db: &Db, id: Uuid) -> Result<usize, Error<Null>> {
    db.run(move |conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .await
        .map_err(ApiResponse::from_error)
}

pub async fn inject_user(db: &Db, user: User) -> Result<usize, DieselError> {
    db.run(move |conn| diesel::insert_into(users::table).values(user).execute(conn))
        .await
}

/// Returns all [`PublicUser`] information in a vector of users with whom the user (requester)
/// shares a workspace.
/// 
/// What the function does (in a single database transaction):
/// - Collects the IDs of workspaces of which the user is a member in a vector of [`Uuid`]s
/// - Collects users who are a member of any of the workspaces in a vector of [`User`]s
/// - Returns the vector of [`User`]s into a vector of [`PublicUser`]s
pub async fn get_user_ids_in_same_workspaces(db: &Db, user: Uuid) -> Result<Vec<Uuid>, Error<Null>> {
    use crate::schema::{
        users, users::dsl as users_dsl, workspace_members::dsl as workspace_members_dsl,
    };

    db.run(move |conn| {
        conn.transaction::<Vec<Uuid>, diesel::result::Error, _>(|conn| {
            // 1. Get workspace IDs where the user is a member
            let workspace_ids: Vec<Uuid> = workspace_members_dsl::workspace_members
                .filter(workspace_members_dsl::member.eq(user))
                .select(workspace_members_dsl::workspace)
                .load(conn)?;

            // No shared workspaces => no users
            if workspace_ids.is_empty() {
                return Ok(vec![]);
            }

            // 2. Find users who are members of those workspaces
            let users_found = workspace_members_dsl::workspace_members
                .inner_join(users_dsl::users.on(users_dsl::id.eq(workspace_members_dsl::member)))
                .filter(workspace_members_dsl::workspace.eq_any(&workspace_ids))
                .select(users::id)
                .load::<Uuid>(conn)?;

            Ok(users_found)
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}
