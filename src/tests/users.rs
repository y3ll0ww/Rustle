use rocket::{
    http::{ContentType, Status},
    local::{
        asynchronous::{Client as AsyncClient, LocalResponse as AsyncLocalResponse},
        blocking::{Client, LocalResponse},
    },
};

use crate::{
    cookies::TOKEN_COOKIE,
    forms::login::LoginForm,
    routes::USERS,
};

#[cfg(test)]
mod deleting_users;
#[cfg(test)]
mod getting_users;
#[cfg(test)]
mod injecting_users;
#[cfg(test)]
mod invitation_flow;
#[cfg(test)]
mod login_logout;

fn route_users_all() -> String {
    let mut route = USERS.to_string();
    route.pop(); // Remove the tailing slash; it will invalidate the endpoint
    route
}

fn route_users_by_name(username: &str) -> String {
    format!("{USERS}{username}")
}

fn route_users_admin_inject_users() -> String {
    format!("{USERS}create")
}

fn route_users_browse(status: Option<u16>, role: Option<u16>) -> String {
    let pagination = match (status, role) {
        (Some(status), Some(role)) => format!("?status={status}&role={role}"),
        (Some(status), None) => format!("?status={status}"),
        (None, Some(role)) => format!("?role={role}"),
        (None, None) => String::new(),
    };

    format!("{USERS}browse{pagination}")
}

fn route_users_delete(id: &str) -> String {
    format!("{USERS}delete/{id}")
}

fn route_users_login() -> String {
    format!("{USERS}login")
}

fn route_users_logout() -> String {
    format!("{USERS}logout")
}

const ROUTE_GET: &str = "/user/";
const ROUTE_INVITE_GET: &str = "/user/invite/get/";
const ROUTE_INVITE_SET: &str = "/user/invite/set/";

const ADMIN_USERNAME: &str = "admin";
const ADMIN_PASSWORD: &str = "admin_password123";

const DEFAULT_USERNAME: &str = "test_user";
const DEFAULT_PASSWORD: &str = "strong_password";

const INVITED_USER_1_FIRST_NAME: &str = "Lucas";
const INVITED_USER_1_LAST_NAME: &str = "Bennett";
const INVITED_USER_1_USERNAME: &str = "lucas_bennett";
const INVITED_USER_1_EMAIL_ADDR: &str = "lucas.benett@example.com";

const INVITED_USER_2_FIRST_NAME: &str = "Ava";
const INVITED_USER_2_LAST_NAME: &str = "Thornton";
const INVITED_USER_2_USERNAME: &str = "ava_thornton";
const INVITED_USER_2_EMAIL_ADDR: &str = "ava.thornton@example.com";

const INVITED_USER_3_FIRST_NAME: &str = "Mia";
const INVITED_USER_3_LAST_NAME: &str = "Delgado";
const INVITED_USER_3_EMAIL_ADDR: &str = "mia.delgado@example.com";

const DUPLICATE_USER_1_EMAIL_ADDR: &str = "lucas_bennett_1@example.com";
const DUPLICATE_USER_2_EMAIL_ADDR: &str = "ava_thornton_1@example.com";

pub const ADMIN_LOGIN: LoginForm = LoginForm {
    username: ADMIN_USERNAME,
    password: ADMIN_PASSWORD,
};

pub const DEFAULT_LOGIN: LoginForm = LoginForm {
    username: DEFAULT_USERNAME,
    password: DEFAULT_PASSWORD,
};

pub const INVITED_USER_1_LOGIN: LoginForm = LoginForm {
    username: INVITED_USER_1_USERNAME,
    password: DEFAULT_PASSWORD,
};

pub const INVITED_USER_2_LOGIN: LoginForm = LoginForm {
    username: INVITED_USER_2_USERNAME,
    password: DEFAULT_PASSWORD,
};

pub fn login(client: &Client, login_form: LoginForm) {
    let login_response = client
        .post(route_users_login())
        .header(ContentType::Form)
        .body(login_form.body())
        .dispatch();

    // Assert the login request was successful
    assert_eq!(login_response.status(), Status::Ok);

    // Assert that the cookies are added
    assert_authorized_cookies(login_response, true);
}

pub fn logout(client: &Client) {
    let logout_response = client.post(route_users_logout()).dispatch();

    // Assert that the logout request was successful
    assert_eq!(logout_response.status(), Status::Ok);

    // Assert that the cookies are removed
    assert_authorized_cookies(logout_response, false);
}

fn assert_authorized_cookies(response: LocalResponse<'_>, available: bool) {
    // Get the cookies after the response
    let cookies = response.cookies();
    let token_cookie = cookies.get_private(TOKEN_COOKIE);

    // Perform the assertions on the cookies based on provided boolean
    if available {
        assert!(token_cookie.is_some());
    } else {
        assert!(token_cookie.is_none());
    }
}

pub async fn async_login(client: &AsyncClient, login_form: LoginForm<'static>) {
    let login_response = client
        .post(route_users_login())
        .header(ContentType::Form)
        .body(login_form.body())
        .dispatch()
        .await;

    // Assert the login request was successful
    assert_eq!(login_response.status(), Status::Ok);

    // Assert that the cookies are added
    async_assert_authorized_cookies(login_response, true);
}

fn async_assert_authorized_cookies(response: AsyncLocalResponse<'_>, available: bool) {
    // Get the cookies after the response
    let cookies = response.cookies();
    let token_cookie = cookies.get_private(TOKEN_COOKIE);

    // Perform the assertions on the cookies based on provided boolean
    if available {
        assert!(token_cookie.is_some());
    } else {
        assert!(token_cookie.is_none());
    }
}
