use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

use crate::models::users::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertedUser {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl InsertedUser {
    pub fn from_user(user: &User) -> Self {
        InsertedUser {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NewUser<'v> {
    #[field(validate = len(1..))]
    pub username: &'v str,
    pub display_name: Option<&'v str>,
    pub password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'v str,
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
