use rocket::State;
use uuid::Uuid;

use crate::api::{ApiResponse, Error, Null};

use super::RedisMutex;

pub const CACHE_INVITE_TOKEN: &str = "invite_token:";
pub const CACHE_TTL_24_HOURS: Option<u64> = Some(86400);

pub fn cache_key_invite_token(token: &str) -> String {
    format!("{CACHE_INVITE_TOKEN}{token}")
}

pub async fn add_invite_token(
    redis: &State<RedisMutex>,
    token: &str,
    user_id: Uuid,
) -> Result<(), Error<Null>> {
    redis
        .lock()
        .await
        .set_to_cache(&cache_key_invite_token(token), &user_id, CACHE_TTL_24_HOURS)
        .await
}

pub async fn get_invite_token(redis: &State<RedisMutex>, token: &str) -> Result<Uuid, Error<Null>> {
    match redis
        .lock()
        .await
        .get_from_cache(&cache_key_invite_token(token))
        .await?
    {
        Some(value) => Ok(value),
        None => Err(ApiResponse::not_found(
            "Key not found; possibly expired".to_string(),
        )),
    }
}

pub async fn remove_invite_token(
    redis: &State<RedisMutex>,
    token: &str,
) -> Result<(), Error<Null>> {
    redis
        .lock()
        .await
        .remove_from_cache(&cache_key_invite_token(token))
        .await
}
