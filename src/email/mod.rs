pub mod builder;
pub mod smtp;
pub mod templates;

use builder::MailBuilder;
use lettre::transport::smtp::response::Response;
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

    /// Uses a [`template`](MailTemplate) email to be [`send`](Self::send).
    /// 
    /// # Parameters
    /// 
    /// * [`recipient`](PublicUser): The information for the receiver of the email.
    /// * [`template`](MailTemplate): The content of the email to be send.
    /// 
    /// # Returns
    /// 
    /// This function returns the [`send`](Self::send) function.
    pub fn send_template(
        &self,
        recipient: &PublicUser,
        template: MailTemplate,
    ) -> Result<Response, String> {
        self.send(recipient, &template.subject, template.body())
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
    pub fn send(
        &self,
        recipient: &PublicUser,
        subject: &str,
        body: String,
    ) -> Result<Response, String> {
        // Build the message to send
        let message = self
            .mail
            .build(recipient)
            .subject(subject)
            .body(body)
            .map_err(|e| e.to_string())?;

        // Send the message using SMTP configuration
        self.smtp.send(message).map_err(|e| e.to_string())
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

    let template = templates::MailTemplate::invitation(&inviter, &recipient, "ATT Test Tool");

    let result = MailClient::no_reply().send_template(&recipient, template);
    println!("{result:?}");
    assert!(result.is_ok());
}
