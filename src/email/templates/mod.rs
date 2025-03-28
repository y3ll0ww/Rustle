use crate::models::users::PublicUser;

pub mod invitation;

pub struct MailTemplate {
    pub subject: String,
    content: String,
}

impl MailTemplate {
    pub fn body(&self) -> String {
        format!("{}{}", self.content, Self::footer())
    }

    fn footer() -> String {
        format!("<div><h4>Footer</h4><p>To be used for all email templates.</p></div>")
    }
}
