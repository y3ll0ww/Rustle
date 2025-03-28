use std::collections::HashMap;

use lettre::message::SinglePart;
use regex::Regex;

use super::Asset;

pub struct WebElement {
    pub html: String,
    pub source: SinglePart,
}

impl Asset for WebElement {}

impl WebElement {
    pub fn singlepart(
        file_name: &str,
        replacements: HashMap<&str, String>,
    ) -> Result<SinglePart, String> {
        let re = Regex::new(r"\{\{\s*([\w]+)\s*\}\}").unwrap();
    
        let html_content = Self::read_element(file_name)?;
    
        // First, check if any placeholder is missing in replacements
        for caps in re.captures_iter(&html_content) {
            let key = caps[1].to_uppercase();
            if !replacements.contains_key(key.as_str()) {
                return Err(format!("Error: Missing key for placeholder '{{{{ {} }}}}'", &caps[1]));
            }
        }

        // Replace placeholders with values
        let result = re.replace_all(&html_content, |caps: &regex::Captures| {
            let key = caps[1].to_uppercase();
            replacements.get(key.as_str()).unwrap().clone() // Safe unwrap since we checked earlier
        });
    
        // Convert result to String and return
        Ok(SinglePart::html(result.into_owned()))
    }
}
