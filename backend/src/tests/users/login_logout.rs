use rocket::http::ContentType;

use crate::{forms::login::LoginForm, tests::{
    response_not_found, response_ok, response_unauthorized, test_client,
    users::{
        login, logout, route_users_by_name, route_users_login, route_users_logout, ADMIN_LOGIN,
        DEFAULT_LOGIN, INVITED_USER_2_USERNAME,
    },
}};

use super::INVITED_USER_2_LOGIN;

#[test]
fn login_then_logout_admin() {
    login_then_logout(ADMIN_LOGIN);
}

#[test]
fn login_user_then_logout() {
    login_then_logout(DEFAULT_LOGIN);
}

fn login_then_logout(login_form: LoginForm) {
    let client = test_client();
    login(&client, login_form);
    logout(&client);
}

#[test]
fn logout_without_being_logged_in() {
    let client = test_client();
    response_unauthorized(client.post(route_users_logout()));
}

#[test]
fn login_attempt_by_invited_user() {
    let client = test_client();

    // Make sure the user exists:
    // a) Login as admin
    login(&client, ADMIN_LOGIN);

    // b) Get the invited user by username and assert it exists
    response_ok(client.get(route_users_by_name(INVITED_USER_2_USERNAME)));

    // d) Logout
    logout(&client);

    // Attempt login as invited user
    response_not_found(
        client
            .post(route_users_login())
            .header(ContentType::Form)
            .body(INVITED_USER_2_LOGIN.body()),
    );
}
