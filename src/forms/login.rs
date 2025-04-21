use serde::{Deserialize, Serialize};

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
