use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

use crate::schema::workspaces;

/// This struct represents the information required to create a new
/// [`Workspace`](crate::models::workspaces::Workspace) via a form.
#[derive(Debug, Deserialize, FromForm, Serialize)]
pub struct NewWorkspaceForm {
    #[field(validate = len(1..))]
    pub name: String,
    pub description: Option<String>,
}

impl NewWorkspaceForm {
    pub fn body(&self) -> String {
        format!(
            "name={}&description={}",
            self.name,
            self.description.as_deref().unwrap_or_default(),
        )
    }
}

/// This struct represents the information required to update the basic information of an existing
/// [`Workspace`](crate::models::workspaces::Workspace) via a form.
#[derive(AsChangeset, Clone, Debug, Deserialize, FromForm, Serialize)]
#[diesel(table_name = workspaces)]
pub struct UpdateWorkspaceForm {
    #[field(validate = len(1..))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl UpdateWorkspaceForm {
    pub fn body(&self) -> String {
        format!(
            "name={}&description={}&image_url={}",
            self.name.as_deref().unwrap_or_default(),
            self.description.as_deref().unwrap_or_default(),
            self.image_url.as_deref().unwrap_or_default(),
        )
    }
}
