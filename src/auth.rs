use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

use crate::models::users::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // User ID
    role: UserRole, // User role
    exp: usize, // Expiration timestamp
}

pub struct AuthenticatedUser {
    pub id: String,
    pub role: UserRole,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => return Outcome::Error((Status::Unauthorized, "Missing or invalid token".to_string())),
        };
        // QWERTY add the secret key here
        let secret = "your-secret-key".as_bytes();
        match decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default()) {
            Ok(token_data) => request::Outcome::Success(AuthenticatedUser {
                id: token_data.claims.sub,
                role: token_data.claims.role,
            }),
            Err(e) => Outcome::Error((Status::Unauthorized, format!("Invalid token: {}", e))),
        }
    }
}

pub fn create_token(user_id: String, privilege: i32) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: UserRole::try_from(privilege)?,
        exp: expiration,
    };
    // QWERTY add the secret key here
    encode(&Header::default(), &claims, &EncodingKey::from_secret("your-secret-key".as_bytes()))
        .map_err(|e| format!("Failed to create token: {}", e))
}