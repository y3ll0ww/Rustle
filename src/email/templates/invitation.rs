use std::collections::HashMap;

use lettre::message::MultiPart;

use crate::email::assets::elements::HtmlElement;

use super::*;

impl MailTemplate {
    pub fn invitation(
        inviter: &PublicUser,
        recipient: &PublicUser,
        team_name: &str,
        token: &str,
    ) -> Result<Self, String> {
        let inviter_name = inviter.get_name();
        let link = format!("https://localhost/set-password?token={token}");

        Ok(MailTemplate {
            subject: format!("{inviter_name} invited you to join {team_name}"),
            content: HtmlElement::invitation(recipient, inviter, "ATT Test Tool", &link)?,
        })
    }
}

impl HtmlElement {
    fn invitation(
        recipient: &PublicUser,
        inviter: &PublicUser,
        team_name: &str,
        link: &str,
    ) -> Result<MultiPart, String> {
        let replacements = HashMap::from([
            ("RECIPIENT", recipient.get_name()),
            ("INVITER", inviter.get_name()),
            ("TEAM_NAME", team_name.to_string()),
            ("INVITE_LINK", link.to_string()),
        ]);

        let html_content = Self::singlepart("invitation.html", replacements)?;

        Ok(MultiPart::alternative().singlepart(html_content))
    }
}
