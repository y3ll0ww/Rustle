use std::str::FromStr;

use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MessageBuilder};
use lettre::{Address, Message};

use crate::models::users::PublicUser;

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

    /// QWERTY UNWRAP USED: ERROR HANDLING NEEDED
    pub fn build(&self, recipient: &PublicUser) -> MessageBuilder {
        Message::builder()
            .date_now()
            .header(ContentType::TEXT_HTML)
            .from(self.from_mailbox().unwrap())
            .to(Self::to_mailbox(recipient).unwrap())
    }

    fn from_mailbox(&self) -> Result<Mailbox, AddressError> {
        let address = Address::new(self.user.clone(), self.domain.clone())?;

        let mailbox = Mailbox {
            name: Some(self.name.clone()),
            email: address,
        };

        Ok(mailbox)
    }

    fn to_mailbox(recipient: &PublicUser) -> Result<Mailbox, AddressError> {
        let address = Address::from_str(&recipient.email)?;

        let mailbox = Mailbox {
            name: Some(recipient.get_name()),
            email: address,
        };

        Ok(mailbox)
    }
}
