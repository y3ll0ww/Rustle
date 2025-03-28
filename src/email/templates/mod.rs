use lettre::message::{MultiPart, SinglePart};

use crate::{email::assets::attachments::InlineAttachment, models::users::PublicUser};

pub mod invitation;

pub struct MailTemplate {
    pub subject: String,
    pub content: MultiPart,
}

impl MailTemplate {
    pub fn generate(&self) -> Result<MultiPart, String> {
        Ok(MultiPart::mixed()
            .multipart(self.content.clone())
            .multipart(Self::footer()?))
    }

    fn footer() -> Result<MultiPart, String> {
        let asset = InlineAttachment::logo_small()?;

        Ok(MultiPart::alternative().multipart(
            MultiPart::related()
                .singlepart(SinglePart::html(String::from(format!(
                    r#"<footer>
                    <style>
                            /* Basic styles for the footer */
                            footer {{
                                background-color: #333;
                                color: white;
                                padding: 20px;
                                display: flex;
                                justify-content: space-between;
                }}

                            /* Style for the two columns */
                            .footer-column {{
                                width: 45%;
                }}

                            /* Optional: make the text smaller */
                            .footer-column p {{
                                font-size: 14px;
                }}

                            /* Make the footer responsive */
                            @media (max-width: 600px) {{
                                footer {{
                                    flex-direction: column;
                                    align-items: center;
                }}

                                .footer-column {{
                                    width: 100%;
                                    text-align: center;
                                    margin-bottom: 10px;
                }}
                }}
                        </style>
                      <div class="footer-column">{}</div>

                      <div class="footer-column">
                        <h4>Contact</h4>
                        <p>Email: contact@example.com</p>
                        <p>Phone: (123) 456-7890</p>
                      </div>
                    </footer>"#,
                    asset.html
                ))))
                .singlepart(asset.source),
        ))
    }
}
