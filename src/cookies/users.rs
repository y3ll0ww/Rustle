use rocket::http::{Cookie, CookieJar};

use crate::{
    api::{ApiResponse, Error, Null},
    auth::{token_user_info, UserInfo},
};

use super::{get_cookie, TOKEN_COOKIE, USER_COOKIE};

pub async fn generate_and_add_cookies(
    user_id: String,
    username: String,
    privilege: i32,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<Null>> {
    // Pass user info to create token for caching
    let (token, user_info) = token_user_info(user_id.clone(), username.clone(), privilege)
        .await
        .map_err(|e| ApiResponse::internal_server_error(e))?;

    let token_cookie = Cookie::new(TOKEN_COOKIE, token);
    let user_cookie = Cookie::new(USER_COOKIE, user_info);

    cookies.add_private(token_cookie);
    cookies.add_private(user_cookie);

    Ok(())
}

pub async fn get_user_info(cookies: &CookieJar<'_>) -> Result<UserInfo, Error<String>> {
    Ok(get_cookie::<UserInfo>(USER_COOKIE, cookies)?)
}
