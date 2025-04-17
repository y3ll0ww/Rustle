use rocket::http::{ContentType, Status};

use crate::tests::{
    test_client,
    users::{
        login, logout, ADMIN_LOGIN, DEFAULT_LOGIN, INVITED_USER_2_USERNAME, ROUTE_GET, ROUTE_LOGOUT,
    },
};

use super::{INVITED_USER_2_LOGIN, ROUTE_LOGIN};

#[test]
fn login_existing_user_then_logout() {
    let client = test_client();

    // Log in
    login(&client, DEFAULT_LOGIN);

    // Log out
    logout(&client);
}

#[test]
fn logout_without_being_logged_in() {
    let client = test_client();

    // Log out
    let logout_response = client.post(ROUTE_LOGOUT).dispatch();

    // Assert that the logout request returned "Unauthorized"
    assert_eq!(logout_response.status(), Status::Unauthorized);
}

#[test]
fn login_attempt_by_invited_user() {
    let client = test_client();

    // Make sure the user exists:
    // a) Login as admin
    login(&client, ADMIN_LOGIN);

    // b) Get the invited user by username
    let response = client
        .get(format!("{ROUTE_GET}{INVITED_USER_2_USERNAME}"))
        .dispatch();

    // c) Assert it exists
    assert_eq!(response.status(), Status::Ok);

    // d) Logout
    logout(&client);

    // Attempt login as invited user
    let response = client
        .post(ROUTE_LOGIN)
        .header(ContentType::Form)
        .body(INVITED_USER_2_LOGIN.body())
        .dispatch();

    // Since the user status is not active, it will return not found
    assert_eq!(response.status(), Status::NotFound);
}
