use chrono::NaiveDateTime;
use rocket::http::{Cookie, CookieJar};
use uuid::Uuid;

use crate::api::Error;

use super::{get_cookie, WORKSPACE_COOKIE};

pub fn cookie_key_workspace_timestamp(workspace: Uuid) -> String {
    format!("{WORKSPACE_COOKIE}{workspace}")
}

pub fn add_workspace_timestamp(
    workspace: Uuid,
    timestamp: NaiveDateTime,
    cookies: &CookieJar<'_>,
) {
    let cookie = Cookie::new(
        cookie_key_workspace_timestamp(workspace),
        timestamp.to_string(),
    );

    cookies.add_private(cookie);
}

pub fn get_workspace_timestamp(
    workspace: Uuid,
    cookies: &CookieJar<'_>,
) -> Result<NaiveDateTime, Error<String>> {
    get_cookie::<NaiveDateTime>(&cookie_key_workspace_timestamp(workspace), cookies)
}

pub fn remove_workspace_update_cookie(workspace: Uuid, cookies: &CookieJar<'_>) {
    cookies.remove_private(cookie_key_workspace_timestamp(workspace));
}
