use rocket::State;

use crate::{
    api::{ApiResponse, Error, Null},
    models::users::PublicUser,
};

use super::RedisMutex;

pub const CACHE_INVITE_TOKEN: &str = "invite_token:";
pub const CACHE_TTL_24_HOURS: Option<u64> = Some(86400);

pub fn cache_key_invite_token(token: &str) -> String {
    format!("{CACHE_INVITE_TOKEN}{token}")
}

pub async fn add_invite_token(
    redis: &State<RedisMutex>,
    token: &str,
    user: PublicUser,
) -> Result<(), Error<Null>> {
    redis
        .lock()
        .await
        .set_to_cache(&cache_key_invite_token(token), &user, CACHE_TTL_24_HOURS)
        .await
}

pub async fn get_invite_token(
    redis: &State<RedisMutex>,
    token: &str,
) -> Result<PublicUser, Error<Null>> {
    match redis
        .lock()
        .await
        .get_from_cache::<PublicUser>(&cache_key_invite_token(token))
        .await?
    {
        Some(user) => Ok(user),
        None => Err(ApiResponse::not_found(
            "Key not found; possibly expired".to_string(),
        )),
    }
}
