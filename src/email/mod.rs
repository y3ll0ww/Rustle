pub mod assets;
pub mod builder;
pub mod smtp;
pub mod templates;

use builder::MailBuilder;
use lettre::{transport::smtp::response::Response, Message};
use smtp::Smtp;
use templates::MailTemplate;

use crate::models::users::PublicUser;

pub struct MailClient {
    smtp: Smtp,
    mail: MailBuilder,
}

impl MailClient {
    /// Constructs a new [`MailClient`].
    ///
    /// # Parameters
    ///
    /// * [`mail`](MailBuilder): For building the message to be send with particular information.
    ///
    /// # Returns
    ///
    /// * [`Self`](MailClient): An instance of `Self` with the [`default`](Smtp::default) settings
    ///   for [`SMTP`](Smtp).
    pub fn new(mail: MailBuilder) -> Self {
        MailClient {
            smtp: Smtp::default(),
            mail,
        }
    }

    /// Creates a [`new`](Self::new) instance for the **noreply** user.
    pub fn no_reply() -> Self {
        Self::new(MailBuilder::new("noreply"))
    }

    pub fn send_invitation(
        &self,
        inviter: &PublicUser,
        recipient: &PublicUser,
        team_name: &str,
    ) -> Result<Response, String> {
        let template = MailTemplate::invitation(inviter, recipient, team_name)?;
        let message = self.mail.from_template(recipient, template)?;
        self.send(message)
    }

    /// Sends a generated email with the provided information.
    ///
    /// # Parameters
    ///
    /// * [`recipient`](PublicUser): The information for the receiver of the email.
    /// * [`subject`](String): The subject of the email.
    /// * [`body`](String): The content body of the email.
    ///
    /// # Returns
    ///
    /// * [`Response`]: The response when the email gets sent successfully.
    /// * [`String`]: The error for building or sending the message, converted to a string.
    pub fn send(&self, message: Message) -> Result<Response, String> {
        // Send the message using SMTP configuration
        self.smtp.send(message)
    }
}

#[test]
fn send_mail_test() {
    let recipient = PublicUser {
        id: uuid::Uuid::new_v4(),
        role: 0,
        username: String::from("mohammad_nouranian"),
        display_name: Some(String::from("Mohammad Nouranian")),
        email: String::from("mohammad_nouranian@legrand_ext.com"),
        bio: None,
        avatar_url: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let inviter = PublicUser {
        id: uuid::Uuid::new_v4(),
        role: 10,
        username: String::from("André Cybulski"),
        display_name: None,
        email: String::from("andre_cybulski@ecotap.eu"),
        bio: None,
        avatar_url: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = MailClient::no_reply().send_invitation(&inviter, &recipient, "ATT Test Tool");

    println!("{result:?}");
    assert!(result.is_ok());
}
