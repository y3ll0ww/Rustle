use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct Password<'v> {
    #[field(validate = len(6..))]
    #[field(validate = eq(self.second))]
    pub first: &'v str,
    #[field(validate = eq(self.first))]
    pub second: &'v str,
}

impl Password<'_> {
    pub fn body(&self) -> String {
        format!("first={}&second={}", self.first, self.second)
    }

    pub fn generate(password: Option<&str>) -> Result<String, argon2::password_hash::Error> {
        // Use input or a generated UUID
        let password = match password {
            Some(password) => password.to_string(),
            None => Uuid::new_v4().to_string(),
        };

        // Create a password instance
        let password = Password {
            first: &password,
            second: &password,
        };

        // Hash it
        password.hash_password()
    }

    pub fn verify_password(input_password: &str, stored_hash: &str) -> Result<bool, Error> {
        let password = input_password.as_bytes();
        let hash = PasswordHash::new(stored_hash)?;
        Ok(Argon2::default().verify_password(password, &hash).is_ok())
    }

    pub fn inputs_match(&self) -> bool {
        self.first == self.second
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
