use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

use crate::schema::teams;

/// This struct represents the information required to create a new
/// [`Team`](crate::models::teams::Team) via a form.
#[derive(Debug, Deserialize, FromForm, Serialize)]
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

/// This struct represents the information required to update the basic information of an existing
/// [`Team`](crate::models::teams::Team) via a form.
#[derive(AsChangeset, Clone, Debug, Deserialize, FromForm, Serialize)]
#[diesel(table_name = teams)]
pub struct UpdateTeamForm {
    #[field(validate = len(1..))]
    pub team_name: Option<String>,
    pub team_description: Option<String>,
    pub image_url: Option<String>,
}

impl UpdateTeamForm {
    pub fn body(&self) -> String {
        format!(
            "team_name={}&description={}&image_url={}",
            self.team_name.as_deref().unwrap_or_default(),
            self.team_description.as_deref().unwrap_or_default(),
            self.image_url.as_deref().unwrap_or_default(),
        )
    }
}
