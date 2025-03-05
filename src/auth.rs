//! Authentication and Authorization Module
//!
//! This module handles user authentication, token generation, and Redis-based session management.
//! It includes JWT-based authentication and role-based access control.
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{cookies::TOKEN_COOKIE, models::users::UserRole};

const TOKEN_VALIDITY_HRS: i64 = 24;

// QWERTY find better solution for handling secret key
pub const SECRET: &str = "BquiyC07WQ27ldPF0FuVmqS6arSPs76MwBu895qQnjM=";

pub struct JwtGuard;

#[async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Attempt to extract the JWT token from cookies or Authorization header
        let cookies = request.cookies();
        let token_cookie = cookies.get_private(TOKEN_COOKIE);

        let token = match token_cookie {
            Some(cookie) => format!("{}", cookie.value()),
            // If no token is found in cookies, look for it in the Authorization header
            None => match request.headers().get_one("Authorization") {
                Some(header_value) if header_value.starts_with("Bearer ") => {
                    header_value[7..].to_string() // Extract the token part (skip "Bearer ")
                }
                _ => {
                    return Outcome::Error((Status::Unauthorized, "No token provided".to_string()))
                }
            },
        };

        // Validate the token by decoding it and checking the expiration
        match Claims::decode_and_validate(&token) {
            Ok(_) => Outcome::Success(JwtGuard),
            Err(e) => Outcome::Error(e),
        }
    }
}

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
    /// This function now only decodes the JWT and checks its expiration, with no Redis involved.
    pub fn decode_and_validate(token: &str) -> Result<TokenData<Self>, (Status, String)> {
        // Decode the token using the secret key
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_bytes()),
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

/// Generates a JWT token and a [`UserInfo`] JSON, and returns them in a tuple of Strings.
///
/// # Arguments
/// * `user_id` - The user ID.
/// * `username` - The username.
/// * `privilege` - The user's role privilege.
///
/// # Returns
/// * `Ok((String, String))` containing the JWT token and a serialize [`UserInfo`] if successful.
/// * `Err(String)` if token generation or deserialization fails, or if the [`UserRole`] cannot be
///   downcasted from the privilege field.
pub async fn token_user_info(
    user_id: String,
    username: String,
    privilege: i32,
) -> Result<(String, String), String> {
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
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .map_err(|e| format!("Failed to create token: {}", e))?;

    // Cache user info
    let user_info = UserInfo {
        id: user_id.clone(),
        username,
        role: UserRole::try_from(privilege)?,
    };

    let user_info_json = serde_json::to_string(&user_info)
        .map_err(|e| format!("Failed to serialize user info: {}", e))?;

    Ok((token, user_info_json))
}
