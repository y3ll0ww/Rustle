use chrono::Utc;
use rocket::http::ContentType;
use uuid::Uuid;

use super::{ADMIN_PASSWORD, ADMIN_USERNAME, DEFAULT_PASSWORD, DEFAULT_USERNAME};
use crate::{
    forms::password::Password,
    models::users::{User, UserRole, UserStatus},
    tests::{response_ok, test_client, users::route_users_admin_inject_users},
};

#[test]
fn inject_custom_user() {
    user_injection(
        "invited_user",
        "Invited",
        "User",
        Some("password123!"),
        "invited_user@example.com",
        UserRole::Reviewer,
        UserStatus::Invited,
    )
}

#[test]
fn inject_admin_user() {
    user_injection(
        ADMIN_USERNAME,
        "Admin",
        "User",
        Some(ADMIN_PASSWORD),
        "admin@example.com",
        UserRole::Admin,
        UserStatus::Active,
    )
}

#[test]
fn inject_default_user() {
    user_injection(
        DEFAULT_USERNAME,
        "Default",
        "User",
        Some(DEFAULT_PASSWORD),
        "test_user@example.com",
        UserRole::Reviewer,
        UserStatus::Active,
    )
}

fn user_injection(
    username: &str,
    first_name: &str,
    last_name: &str,
    password: Option<&str>,
    email: &str,
    role: UserRole,
    status: UserStatus,
) {
    let client = test_client();

    // Construct a JSON payload matching the User structure
    let user = User {
        id: Uuid::new_v4(),
        username: username.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
        phone: None,
        role: i16::from(role),
        status: i16::from(status),
        job_title: None,
        password: Password::generate(password).unwrap(),
        bio: None,
        avatar_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    // Turn into a payload string
    let payload = serde_json::to_string(&user).unwrap();

    response_ok(
        client
            .post(route_users_admin_inject_users())
            .header(ContentType::JSON)
            .body(payload),
    );
}
