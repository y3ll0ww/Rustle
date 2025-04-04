use lettre::message::MultiPart;

use crate::models::users::PublicUser;

use super::assets::elements::HtmlElement;

pub mod invitation;

pub struct MailTemplate {
    pub subject: String,
    pub content: MultiPart,
}

impl MailTemplate {
    pub fn generate(&self) -> Result<MultiPart, String> {
        Ok(MultiPart::mixed()
            .multipart(self.content.clone())
            .multipart(HtmlElement::footer()?))
    }
}
