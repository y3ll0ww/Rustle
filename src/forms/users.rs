use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

/// This struct represents the information required to create a new [`User`] via a form.
#[derive(Debug, Deserialize, FromForm, Serialize)]
pub struct NewUserForm<'v> {
    #[field(validate = len(1..))]
    pub username: &'v str,
    pub password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'v str,
}

impl NewUserForm<'_> {
    pub fn body(&self) -> String {
        format!(
            "username={}&password.first={}&password.second={}&email={}",
            self.username, self.password.first, self.password.second, self.email,
        )
    }
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct LoginForm<'v> {
    pub username: &'v str,
    pub password: &'v str,
}

impl LoginForm<'_> {
    pub fn body(&self) -> String {
        format!("username={}&password={}", self.username, self.password,)
    }
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

impl Password<'_> {
    pub fn verify_password(input_password: &str, stored_hash: &str) -> Result<bool, Error> {
        let password = input_password.as_bytes();
        let hash = PasswordHash::new(stored_hash)?;
        Ok(Argon2::default().verify_password(password, &hash).is_ok())
    }

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
