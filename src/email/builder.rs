use std::str::FromStr;

use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MessageBuilder};
use lettre::{Address, Message};

use crate::models::users::PublicUser;

use super::templates::MailTemplate;

const MAIL_NAME: &str = "Rustle";
const MAIL_DOMAIN: &str = "rustle.com";

pub struct MailBuilder {
    name: String,
    user: String,
    domain: String,
}

impl MailBuilder {
    pub fn new(user: &str) -> Self {
        MailBuilder {
            name: MAIL_NAME.to_string(),
            domain: MAIL_DOMAIN.to_string(),
            user: user.to_string(),
        }
    }

    pub fn builder(&self, recipient: &PublicUser) -> Result<MessageBuilder, String> {
        Ok(Message::builder()
            .date_now()
            .header(ContentType::TEXT_HTML)
            .from(self.from_mailbox()?)
            .to(Self::to_mailbox(recipient)?))
    }

    pub fn from_template(&self, recipient: &PublicUser, template: MailTemplate) -> Result<Message, String> {
        self.builder(recipient)?
            .subject(template.subject.clone())
            .multipart(template.generate()?)
            .map_err(|e| e.to_string())
    }

    fn from_mailbox(&self) -> Result<Mailbox, String> {
        let address =
            Address::new(self.user.clone(), self.domain.clone()).map_err(|e| e.to_string())?;

        let mailbox = Mailbox {
            name: Some(self.name.clone()),
            email: address,
        };

        Ok(mailbox)
    }

    fn to_mailbox(recipient: &PublicUser) -> Result<Mailbox, String> {
        let address = Address::from_str(&recipient.email).map_err(|e| e.to_string())?;

        let mailbox = Mailbox {
            name: Some(recipient.get_name()),
            email: address,
        };

        Ok(mailbox)
    }
}
