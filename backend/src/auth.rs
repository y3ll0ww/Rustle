//! Authentication and Authorization Module
//!
//! This module handles user authentication, token generation, and Redis-based session management.
//! It includes JWT-based authentication and role-based access control.
use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{encode, EncodingKey, Header, TokenData};
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{
    cookies::TOKEN_COOKIE,
    models::users::{PublicUser, User},
};

const TOKEN_VALIDITY_HRS: i64 = 24;

// QWERTY find better solution for handling secret key
pub const SECRET: &str = "BquiyC07WQ27ldPF0FuVmqS6arSPs76MwBu895qQnjM=";

pub struct JwtGuard {
    claims: Claims,
}

impl JwtGuard {
    pub fn get_user(&self) -> PublicUser {
        self.claims.sub.clone()
    }

    pub async fn secure(user: &User, cookies: &CookieJar<'_>) -> Result<(), String> {
        let token = Self::generate_token(PublicUser::from(user)).await?;

        let cookie = Cookie::build((TOKEN_COOKIE, token))
            .http_only(true) // Prevent JavaScript access (mitigates XSS)
            .same_site(SameSite::Lax) // Lax so frontend can reach it. SameSite to prevent CSRF attacks
            .secure(false) // TODO!: Set to 'true': Only send cookie over HTTPS
            .path("/"); // Available site-wide

        cookies.add_private(cookie);

        Ok(())
    }

    async fn generate_token(user: PublicUser) -> Result<String, String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(TOKEN_VALIDITY_HRS))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .map_err(|e| format!("Failed to create token: {e}"))?;

        Ok(token)
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Attempt to extract the JWT token from cookies or Authorization header
        let cookies = request.cookies();
        let token_cookie = cookies.get_private(TOKEN_COOKIE);

        let token = match token_cookie {
            Some(cookie) => cookie.value().to_string(),
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
            Ok(decoded) => Outcome::Success(JwtGuard {
                claims: decoded.claims,
            }),
            Err(e) => Outcome::Error(e),
        }
    }
}

/// Represents the JWT claims stored in a token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The subject of the token (User ID).
    pub sub: PublicUser,
    /// Expiration timestamp (Unix epoch).
    pub exp: usize,
}

impl Claims {
    /// This function decodes the JWT and checks its expiration.
    pub fn decode_and_validate(token: &str) -> Result<TokenData<Claims>, (Status, String)> {
        // Get the payload part from the token:
        // 0 = header
        // 1 = payload
        // 2 = signature
        let token_payload = token.split('.').collect::<Vec<&str>>()[1];

        // Deserialize the payload properly (own implementation because of character escaping)
        let decoded_payload = String::from_utf8_lossy(
            &general_purpose::URL_SAFE_NO_PAD
                .decode(token_payload)
                .map_err(|e| (Status::Unauthorized, e.to_string()))?,
        )
        .to_string();

        // Deserialize the payload to Claims struct
        let claims = serde_json::from_str::<Claims>(&decoded_payload).map_err(|e| {
            (
                Status::Unauthorized,
                format!("Failed to parse claims: {}", e),
            )
        })?;

        // Validate expiration date
        if claims.exp < chrono::Utc::now().timestamp() as usize {
            return Err((Status::Unauthorized, "Token has expired".to_string()));
        }

        // If everything looks good, return the claims in the TokenData struct
        Ok(TokenData {
            claims,
            header: Default::default(),
        })
    }
}
