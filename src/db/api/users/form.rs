use std::fmt::{Display, Formatter};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromForm, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Account<'v> {
    #[field(validate = len(1..))]
    pub username: &'v str,
    pub display_name: Option<&'v str>,
    pub password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'v str,
    pub bio: Option<&'v str>,
    pub avatar_url: Option<&'v str>,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct Password<'v> {
    #[field(validate = len(6..))]
    #[field(validate = eq(self.second))]
    #[allow(unused)]
    pub first: &'v str,
    #[allow(unused)]
    #[field(validate = eq(self.first))]
    pub second: &'v str,
}

impl<'v> Password<'v> {
    pub fn hash_password(&self) -> Result<String, argon2::password_hash::Error> {
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        Ok(argon2
            .hash_password(self.first.as_bytes(), &salt)?
            .to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}
