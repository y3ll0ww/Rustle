use serde::{Deserialize, Serialize};

/// This struct represents the information required to create a new
/// [`Project`](crate::models::projects::Project) via a form.
#[derive(Debug, Deserialize, FromForm, Serialize)]
pub struct NewProjectForm {
    #[field(validate = len(1..))]
    pub name: String,
    pub description: Option<String>,
}

impl NewProjectForm {
    pub fn body(&self) -> String {
        format!(
            "name={}&description={}",
            self.name,
            self.description.as_deref().unwrap_or_default(),
        )
    }
}
