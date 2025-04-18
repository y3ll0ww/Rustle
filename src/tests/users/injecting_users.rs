use chrono::Utc;
use rocket::http::{ContentType, Status};
use uuid::Uuid;

use super::{ADMIN_PASSWORD, ADMIN_USERNAME, DEFAULT_PASSWORD, DEFAULT_USERNAME};
use crate::{
    forms::users::Password,
    models::users::{User, UserRole, UserStatus},
    tests::{test_client, users::ROUTE_CREATE},
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

    let payload = serde_json::to_string(&user).unwrap();

    // Send POST request to the correct endpoint `/users`
    let response = client
        .post(ROUTE_CREATE)
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);

    // Optionally, check the response body for success message
    let response_body = response.into_string().unwrap();
    println!("{response_body}");
    assert!(response_body.contains(&format!("User {} created", user.username)));
}
