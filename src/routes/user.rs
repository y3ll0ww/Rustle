use diesel::prelude::*;

use rocket::{form::Form, http::Status, serde::json::Json};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{forms::user::Account, db::Database, models::user::User, schema::user};

pub fn all() -> Vec<rocket::Route> {
    routes![
        submit,
        create_user,
        get_user,
        delete_user
    ]
}

#[post("/form", data = "<form>")]
pub async fn submit<'r>(db: Database, form: Form<Account<'r>>) -> (Status, String) {
    let password = match form.password.hash_password() {
        Ok(hash) => hash,
        Err(e) => {
            return (
                Status { code: 500 },
                format!("Couldn't hash the password: {e}"),
            )
        }
    };

    let new_user = User::new(
        form.username.to_string(),
        form.display_name.map(|s| s.to_string()),
        form.email.to_string(),
        password,
    );

    // Use Diesel to insert the new user
    let result = db
        .run(move |c| {
            diesel::insert_into(user::table)
                .values(&new_user) // Clone new_user into the closure
                .execute(c) // Pass the connection
        })
        .await;

    match result {
        Ok(_) => (
            Status { code: 200 },
            format!("User {} created", form.username),
        ),
        Err(e) => (Status { code: 500 }, format!("Error creating user: {e}")),
    }
}

#[post("/create", format = "json", data = "<user>")]
pub async fn create_user(db: Database, user: Json<User>) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.user_id = Uuid::new_v4().to_string(); // Generate a new UUID

    // Use Diesel to insert the new user
    let result = db
        .run(move |c| {
            diesel::insert_into(user::table)
                .values(&new_user) // Clone new_user into the closure
                .execute(c) // Pass the connection
        })
        .await;

    match result {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}

#[delete("/delete/<id>")]
pub async fn delete_user(db: Database, id: String) -> String {
    let deleted_count = db
        .run(move |conn| diesel::delete(user::table.filter(user::user_id.eq(id))).execute(conn))
        .await
        .ok();

    println!("{deleted_count:?}");

    if deleted_count.is_some() && deleted_count.unwrap() > 0 {
        "User deleted successfully".to_string()
    } else {
        "User not found".to_string()
    }
}

#[get("/<username>")]
pub async fn get_user(db: Database, username: String) -> String {
    let name = username.clone();
    let user: Option<Json<User>> = db
        .run(move |conn| {
            user::table
                .filter(user::username.eq(username))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();

    match user {
        Some(json) => format!("{:?}", json.0),
        None => format!("'{name}' not found..."),
    }
}
