use std::collections::HashSet;

use diesel::{result::Error as DieselError, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    database::Db,
    models::users::{NewUser, PublicUser, User, UserStatus},
    schema::users,
};

pub async fn create_new_user(db: &Db, new_user: NewUser) -> Result<User, Error<Null>> {
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

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

pub async fn get_user_by_id(db: &Db, id: &Uuid) -> Result<User, Error<Null>> {
    let id = id.clone();

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
    let id = id.clone();

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
