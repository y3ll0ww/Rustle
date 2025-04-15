use std::collections::HashSet;

use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use regex::Regex;
use rocket::form;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::users::User;

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

#[derive(Debug, FromForm)]
pub struct InvitedMultipleUsersForm<'v> {
    // Allow at most 10 invites per request
    #[field(validate = len(1..=10))]
    pub users: Vec<InvitedUserForm<'v>>,
    pub space: &'v str,
}

impl InvitedMultipleUsersForm<'_> {
    pub fn body(&self) -> String {
        // Add the space field and replace any whitespaces
        let mut body = format!("space={}", self.space).replace(' ', "+");

        // Iterate over users and add their bodies
        for (i, user) in self.users.iter().enumerate() {
            let user_index = format!("&users[{i}].");
            let user_body = user.body().replace('&', &user_index);
            body.push_str(&format!("{user_index}{user_body}"));
        }

        // Return
        body
    }

    pub fn get_users_and_base_usernames(&self) -> Result<(Vec<User>, HashSet<String>), String> {
        let mut new_users = Vec::new();
        let mut base_usernames = HashSet::new();

        for user in self.users.iter() {
            // Define the username and the display name
            let display_name = format!("{} {}", user.first_name, user.last_name);
            let username = display_name.to_lowercase().replace(' ', "_");

            base_usernames.insert(username.clone());

            // Generate a hashed password
            let password =
                Password::generate(None).map_err(|e| format!("Coudn't hash password: {e}"))?;

            // Add a new user to be processed
            new_users.push(User::new(
                username,
                Some(display_name),
                user.email.to_string(),
                password,
            ));
        }

        Ok((new_users, base_usernames))
    }
}

#[derive(Debug, FromForm)]
pub struct InvitedUserForm<'v> {
    #[field(validate = InvitedUserForm::validate_name())]
    pub first_name: &'v str,
    #[field(validate = InvitedUserForm::validate_name())]
    pub last_name: &'v str,
    // TODO!: better email validator
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'v str,
}

impl InvitedUserForm<'_> {
    fn validate_name<'v>(value: &str) -> form::Result<'v, ()> {
        // Regex for alphabetic characters and whitespaces only
        let re = Regex::new(r"^[a-zA-Z ]*$").unwrap();

        // Check if the value complies to the regex
        if !re.is_match(value) {
            Err(form::Error::validation(
                "Can only contain characters and spaces.",
            ))?
        }

        // Check if the length is within allowed range
        let min_length = 1;
        let max_length = 20;
        if value.len() < min_length || value.len() > max_length {
            return Err(form::Error::validation(format!(
                "Must be between {min_length} and {max_length} characters long."
            )))?;
        }

        Ok(())
    }

    pub fn body(&self) -> String {
        format!(
            "first_name={}&last_name={}&email={}",
            self.first_name, self.last_name, self.email
        )
        .replace(' ', "+")
    }
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
pub struct LoginForm<'v> {
    pub username: &'v str,
    pub password: &'v str,
}

impl LoginForm<'_> {
    pub fn body(&self) -> String {
        format!("username={}&password={}", self.username, self.password)
    }
}

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
