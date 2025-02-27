//! Authentication and Authorization Module
//!
//! This module handles user authentication, token generation, and Redis-based session management.
//! It includes JWT-based authentication and role-based access control.
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use redis::AsyncCommands;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::models::users::UserRole;
use crate::redis::RedisMutex;

pub const USER_KEY: &str = "user:";
pub const AUTH_TOKEN: &str = "auth_token";
const TOKEN_VALIDITY_HRS: i64 = 24;

/// Represents the JWT claims stored in a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The subject of the token (User ID).
    pub sub: String,
    /// The role of the user.
    pub role: UserRole,
    /// Expiration timestamp (Unix epoch).
    pub exp: usize,
}

impl Claims {
    /// Validates and fetches token data from Redis.
    ///
    /// # Arguments
    /// * `redis` - Redis connection.
    /// * `token` - The JWT token to validate.
    ///
    /// # Returns
    /// * `Ok(TokenData<Claims>)` if the token is valid.
    /// * `Err((Status, String))` if token validation fails.
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

/// Represents user information stored in Redis.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: UserRole,
}

impl UserInfo {
    /// Fetches user information from Redis.
    ///
    /// # Arguments
    /// * `redis` - Redis connection.
    /// * `user_id` - User ID to retrieve data for.
    ///
    /// # Returns
    /// * `Ok(UserInfo)` if user data is found.
    /// * `Err(String)` if fetching fails.
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

/// Represents an authenticated user extracted from a valid JWT token.
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    /// Extracts an authenticated user from an incoming request.
    ///
    /// This function:
    /// 1. Extracts the `Authorization` header.
    /// 2. Validates the JWT token using Redis.
    /// 3. Fetches user info from Redis.
    ///
    /// # Returns
    /// * `Outcome::Success(AuthenticatedUser)` if authentication is successful.
    /// * `Outcome::Error` if the authentication fails.
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

/// Creates and stores a JWT token for a user.
///
/// # Arguments
/// * `user_id` - The user ID.
/// * `username` - The username.
/// * `privilege` - The user's role privilege.
/// * `redis_pool` - The Redis connection pool.
///
/// # Returns
/// * `Ok(String)` containing the JWT token if successful.
/// * `Err(String)` if token generation fails.
pub async fn create_token(
    user_id: String,
    username: String,
    privilege: i32,
    redis_pool: &State<RedisMutex>,
) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(TOKEN_VALIDITY_HRS))
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
        .set_ex(&token, &user_id, 24 * 3600)
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

/// Retrieves a Redis connection from the request state.
///
/// # Arguments
/// 
/// * `request` - The incoming HTTP request.
///
/// # Returns
/// 
/// * `Ok(redis::aio::Connection)` if successful.
/// * `Err((Status, String))` if an error occurs.
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
