use std::collections::HashSet;

use regex::Regex;
use rocket::form;

use crate::models::{
    users::{InvitedUser, UserRole},
    workspaces::WorkspaceRole,
};

use super::password::Password;

#[derive(Debug, FromForm)]
pub struct InvitedMultipleUsersForm<'v> {
    // Allow at most 10 invites per request
    #[field(validate = len(1..=10))]
    pub users: Vec<InvitedUserForm<'v>>,
}

impl InvitedMultipleUsersForm<'_> {
    pub fn body(&self) -> String {
        // Add the space field and replace any whitespaces
        let mut body = String::new();

        // Iterate over users and add their bodies
        for (i, user) in self.users.iter().enumerate() {
            let user_index = format!("&users[{i}].");
            let user_body = user.body().replace('&', &user_index);
            body.push_str(&format!("{user_index}{user_body}"));
        }

        // Remove the first ampersand
        body.remove(0);
        body
    }

    pub fn get_users_and_base_usernames(
        &self,
    ) -> Result<(Vec<InvitedUser>, HashSet<String>), String> {
        let mut new_users = Vec::new();
        let mut base_usernames = HashSet::new();

        for user in self.users.iter() {
            // Define the username and the display name
            let username = format!("{}_{}", user.first_name, user.last_name)
                .to_lowercase()
                .replace(' ', "_")
                .replace('\'', "");

            base_usernames.insert(username.clone());

            // Generate a hashed password
            let password =
                Password::generate(None).map_err(|e| format!("Coudn't hash password: {e}"))?;

            // Add a new user to be processed
            new_users.push(InvitedUser {
                username: username.clone(),
                first_name: user.first_name.to_string(),
                last_name: user.last_name.to_string(),
                email: user.email.to_string(),
                role: i16::from(UserRole::Reviewer),
                status: 0,
                password,
                workspace_role: user.workspace_role,
            });
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
    #[field(validate = InvitedUserForm::validate_email())]
    pub email: &'v str,
    #[field(validate = InvitedUserForm::validate_phone())]
    pub phone: Option<&'v str>,
    #[field(validate = InvitedUserForm::validate_workspace_role())]
    pub workspace_role: i16,
}

impl InvitedUserForm<'_> {
    /// Names must be between a valid range of characters and not contain any illegal characters.
    /// Legal characters are all alphabetic characters, including " ", "-" and "'".
    fn validate_name<'v>(value: &str) -> form::Result<'v, ()> {
        let trimmed = value.trim();

        // Check length of the input
        let min_length = 2;
        let max_length = 20;
        if trimmed.len() < min_length || trimmed.len() > max_length {
            return Err(form::Error::validation(format!(
                "name must be {min_length}-{max_length} characters"
            ))
            .into());
        }

        // Check for illegal characters
        if !trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
        {
            return Err(form::Error::validation("Illegal character in name").into());
        }

        Ok(())
    }

    // Simple email format: matches basic email addresses like:
    /// - user123@example.com
    /// - first.last@my-domain.org
    fn validate_email<'v>(value: &str) -> form::Result<'v, ()> {
        let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w{2,}$")
            .map_err(|e| form::Error::validation(format!("Invalid regex: {e}")))?;

        if !re.is_match(value) {
            return Err(form::Error::validation("Invalid email address").into());
        }

        Ok(())
    }

    /// Digits only, 4 to 15 characters. An optional leading '+' is allowed for international
    /// numbers. No spaces, dashes, or other symbols are permitted.
    /// Examples:
    /// - 0032477123456
    /// - 32477123456
    /// - 123456
    fn validate_phone<'v>(value: &Option<&str>) -> form::Result<'v, ()> {
        let re = Regex::new(r"^\d{4,15}$")
            .map_err(|e| form::Error::validation(format!("Invalid regex: {e}")))?;

        if let Some(phone_number) = value {
            if !re.is_match(phone_number) {
                return Err(form::Error::validation(
                    "Phone number must be 4–15 digits, no symbols",
                )
                .into());
            }
        }

        Ok(())
    }

    fn validate_workspace_role<'v>(value: &i16) -> form::Result<'v, ()> {
        Ok(WorkspaceRole::try_from(*value)
            .map(|_| ())
            .map_err(form::Error::validation)?)
    }

    pub fn body(&self) -> String {
        let phone = if let Some(number) = self.phone {
            format!("&phone={number}")
        } else {
            String::new()
        };

        format!(
            "first_name={}&last_name={}&email={}&workspace_role={}{phone}",
            self.first_name, self.last_name, self.email, self.workspace_role
        )
        .replace(' ', "+")
    }
}
