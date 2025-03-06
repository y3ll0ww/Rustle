use serde::{Deserialize, Serialize};

/// This struct represents the information required to create a new [`User`] via a form.
#[derive(Debug, Deserialize, FromForm, Serialize)]
#[allow(dead_code)]
pub struct NewTeamForm {
    #[field(validate = len(1..))]
    pub team_name: String,
    pub description: Option<String>,
}

impl NewTeamForm {
    pub fn body(&self) -> String {
        format!(
            "team_name={}&description={}",
            self.team_name,
            self.description.as_deref().unwrap_or_default(),
        )
    }
}
