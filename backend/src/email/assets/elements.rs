use std::collections::HashMap;

use lettre::message::{MultiPart, SinglePart};
use regex::Regex;

use super::{attachments::InlineAttachment, Asset};

pub struct HtmlElement {
    pub html: String,
    pub source: SinglePart,
}

impl Asset for HtmlElement {}

impl HtmlElement {
    pub fn footer() -> Result<MultiPart, String> {
        // Get a small logo as inline attachment
        let attachment = InlineAttachment::logo_small()?;

        // Add the attachments HTML in the file
        let replacements = HashMap::from([("LOGO_SMALL", attachment.html.clone())]);
        let html_content = Self::singlepart("footer.html", replacements)?;

        // Return Multipart with the HTML containing an inline attachment
        Ok(MultiPart::alternative().multipart(
            MultiPart::related()
                .singlepart(html_content)
                .singlepart(attachment.source),
        ))
    }

    pub fn singlepart(
        file_name: &str,
        replacements: HashMap<&str, String>,
    ) -> Result<SinglePart, String> {
        // Regex for replacing variables in HTML content.
        // A variable is recognized as {{ VARIABLE }} (case-insensitive)
        let re = Regex::new(r"\{\{\s*([\w]+)\s*\}\}").unwrap();

        // Read the HTML document and extract the text
        let html_content = Self::read_element(file_name)?;

        // Check if any placeholder is missing in replacements
        for caps in re.captures_iter(&html_content) {
            let key = caps[1].to_uppercase();
            if !replacements.contains_key(key.as_str()) {
                return Err(format!(
                    "Error: Missing key for placeholder '{{{{ {} }}}}'",
                    &caps[1]
                ));
            }
        }

        // Replace placeholders with values
        let result = re.replace_all(&html_content, |caps: &regex::Captures| {
            let key = caps[1].to_uppercase();
            replacements.get(key.as_str()).unwrap().clone() // Safe unwrap since we checked earlier
        });

        // Convert result to String and return as SinglePart
        Ok(SinglePart::html(result.into_owned()))
    }
}
