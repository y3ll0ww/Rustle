use std::fs;

use lettre::message::Body;

pub mod attachments;
pub mod elements;

pub trait Asset {
    fn path_root() -> String {
        format!("{}/src/email/assets", env!("CARGO_MANIFEST_DIR"))
    }

    fn read_attachment(file_name: &str) -> Result<Body, String> {
        fs::read(format!("{}/attachments/{file_name}", Self::path_root()))
            .map(|image| Body::new(image))
            .map_err(|e| e.to_string())
    }

    fn read_element(file_name: &str) -> Result<String, String> {
        fs::read_to_string(format!("{}/elements/{file_name}", Self::path_root()))
            .map_err(|e| e.to_string())
    }
}
