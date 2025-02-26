use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use redis::AsyncCommands;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::models::users::UserRole;
use crate::redis::RedisMutex;

pub const USER_KEY: &str = "user:";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // User ID
    pub role: UserRole, // User role
    pub exp: usize,     // Expiration timestamp
}

impl Claims {
    pub async fn fetch_token_data(
        redis: &mut redis::aio::Connection,
        token: &str,
    ) -> Result<TokenData<Self>, (Status, String)> {
        // Check if token exists in Redis
        let token_exists: bool = redis.exists(token).await.map_err(|e| {
            (
                Status::InternalServerError,
                format!("Redis exists failed: {e}"),
            )
        })?;

        if !token_exists {
            return Err((
                Status::Unauthorized,
                format!("Token '{token}' not found in cache"),
            ));
        }

        // Decode JWT
        let secret = "your-secret-key".as_bytes();
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map_err(|e| {
            (
                Status::Unauthorized,
                format!("Invalid token '{token}': {e}"),
            )
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: UserRole,
}

impl UserInfo {
    pub async fn fetch_user_info(
        redis: &mut redis::aio::Connection,
        user_id: &str,
    ) -> Result<Self, String> {
        let user_key = format!("{USER_KEY}{}", user_id);

        let user_info_json: String = redis
            .get(&user_key)
            .await
            .map_err(|e| format!("Redis get failed: {}", e))?;

        serde_json::from_str(&user_info_json)
            .map_err(|e| format!("Failed to parse user info: {}", e))
    }
}

pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract token from headers
        let token = match request.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    "Missing or invalid token".to_string(),
                ))
            }
        };

        let mut redis = match get_cache(&request).await {
            Ok(connection) => connection,
            Err(e) => return Outcome::Error(e),
        };

        let token_data = match Claims::fetch_token_data(&mut redis, token).await {
            Ok(token_data) => token_data,
            Err(e) => return Outcome::Error(e),
        };

        // Fetch user info from Redis
        let user_info = match UserInfo::fetch_user_info(&mut redis, &token_data.claims.sub).await {
            Ok(user) => user,
            Err(e) => return Outcome::Error((Status::InternalServerError, e)),
        };

        Outcome::Success(AuthenticatedUser {
            id: token_data.claims.sub,
            username: user_info.username,
            role: user_info.role,
            token: token.to_string(),
        })
    }
}

pub async fn create_token(
    user_id: String,
    username: String,
    privilege: i32,
    redis_pool: &State<RedisMutex>,
) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.clone(),
        role: UserRole::try_from(privilege)?,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your-secret-key".as_bytes()),
    )
    .map_err(|e| format!("Failed to create token: {}", e))?;

    let mut redis = redis_pool
        .lock()
        .await
        .get_connection()
        .await
        .map_err(|e| format!("Redis connection failed: {}", e))?;

    // Store the token -> user_id mapping
    let _: () = redis
        .set_ex(&token, &user_id, 24 * 3600) // Store user_id as the value
        .await
        .map_err(|e| format!("Redis set token failed: {}", e))?;

    // Cache user info
    let user_info = UserInfo {
        id: user_id.clone(),
        username,
        role: UserRole::try_from(privilege)?,
    };
    let user_info_json = serde_json::to_string(&user_info)
        .map_err(|e| format!("Failed to serialize user info: {}", e))?;

    let user_key = format!("{USER_KEY}{}", user_id);
    let _: () = redis
        .set_ex(user_key, user_info_json, 24 * 3600)
        .await
        .map_err(|e| format!("Redis set user failed: {}", e))?;

    Ok(token)
}

async fn get_cache(request: &Request<'_>) -> Result<redis::aio::Connection, (Status, String)> {
    // Get Redis connection
    let redis_pool = match request.guard::<&State<RedisMutex>>().await {
        Outcome::Success(pool) => pool,
        Outcome::Error((status, _)) => {
            return Err((status, "Failed to access Redis".to_string()));
        }
        Outcome::Forward(_) => {
            return Err((
                Status::InternalServerError,
                "Unexpected forward".to_string(),
            ));
        }
    };

    match redis_pool.lock().await.get_connection().await {
        Ok(connection) => Ok(connection),
        Err(e) => {
            return Err((
                Status::InternalServerError,
                format!("Redis connection failed: {}", e),
            ))
        }
    }
}
