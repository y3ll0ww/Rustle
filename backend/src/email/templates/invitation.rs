use std::collections::HashMap;

use lettre::message::MultiPart;

use crate::email::assets::elements::HtmlElement;

use super::*;

impl MailTemplate {
    pub fn invitation(
        inviter: &PublicUser,
        recipient: &PublicUser,
        workspace_name: &str,
        token: &str,
    ) -> Result<Self, String> {
        let inviter_name = inviter.full_name();
        let link = format!("https://localhost/set-password?token={token}");

        Ok(MailTemplate {
            subject: format!("{inviter_name} invited you to join {workspace_name}"),
            content: HtmlElement::invitation(recipient, inviter, workspace_name, &link)?,
        })
    }
}

impl HtmlElement {
    fn invitation(
        recipient: &PublicUser,
        inviter: &PublicUser,
        workspace_name: &str,
        link: &str,
    ) -> Result<MultiPart, String> {
        let replacements = HashMap::from([
            ("RECIPIENT", recipient.full_name()),
            ("INVITER", inviter.full_name()),
            ("WORKSPACE_NAME", workspace_name.to_string()),
            ("INVITE_LINK", link.to_string()),
        ]);

        let html_content = Self::singlepart("invitation.html", replacements)?;

        Ok(MultiPart::alternative().singlepart(html_content))
    }
}
