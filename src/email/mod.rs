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
        workspace_name: &str,
        token: &str,
    ) -> Result<Response, String> {
        // Get the invitation template
        let template = MailTemplate::invitation(inviter, recipient, workspace_name, token)?;

        // Generate the message
        let message = self.mail.from_template(recipient, template)?;

        // Send the message
        self.smtp.send(message)
    }
}
