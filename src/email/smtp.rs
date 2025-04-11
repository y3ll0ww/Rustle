use lettre::{
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};

pub const SMTP_PORT: u16 = 587;
pub const SMTP_USER: &str = "creola.oconner@ethereal.email";
pub const SMTP_PASS: &str = "5vuWE37G6fbz821vJB";
pub const SMTP_RELAY: &str = "smtp.ethereal.email";

pub struct Smtp {
    relay: String,
    user: String,
    pass: String,
    port: u16,
}

impl Default for Smtp {
    fn default() -> Self {
        Smtp {
            relay: SMTP_RELAY.to_string(),
            user: SMTP_USER.to_string(),
            pass: SMTP_PASS.to_string(),
            port: SMTP_PORT,
        }
    }
}

impl Smtp {
    pub fn send(&self, message: Message) -> Result<Response, String> {
        // Define the credentials
        let credentials = Credentials::new(self.user.clone(), self.pass.clone());

        // Create a SMTP transport
        let transport = SmtpTransport::starttls_relay(&self.relay)
            .map_err(|e| e.to_string())?
            .credentials(credentials)
            .port(self.port)
            .build();

        // Send the email message
        transport.send(&message).map_err(|e| e.to_string())
    }
}
