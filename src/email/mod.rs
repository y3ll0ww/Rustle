pub mod assets;
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

    pub fn send_invitation(
        &self,
        inviter: &PublicUser,
        recipient: &PublicUser,
        team_name: &str,
        token: &str,
    ) -> Result<Response, String> {
        // Get the invitation template
        let template = MailTemplate::invitation(inviter, recipient, team_name, token)?;

        // Generate the message
        let message = self.mail.from_template(recipient, template)?;

        // Send the message
        self.smtp.send(message)
    }
}

#[test]
fn send_mail_test() {
    use crate::models::users::{UserRole, UserStatus};

    let recipient = PublicUser {
        id: uuid::Uuid::new_v4(),
        role: i16::from(UserRole::Reviewer),
        status: i16::from(UserStatus::Invited),
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
        role: i16::from(UserRole::Admin),
        status: i16::from(UserStatus::Active),
        username: String::from("André Cybulski"),
        display_name: None,
        email: String::from("andre_cybulski@ecotap.eu"),
        bio: None,
        avatar_url: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result =
        MailClient::no_reply().send_invitation(&inviter, &recipient, "ATT Test Tool", "token");

    println!("{result:?}");
    assert!(result.is_ok());
}
