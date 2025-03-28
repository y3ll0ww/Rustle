//use lettre::message::{Attachment, MultiPart, SinglePart};
//
//use crate::email::builder::MailBuilder;

use super::*;

impl MailTemplate {
    pub fn invitation(inviter: &PublicUser, recipient: &PublicUser, team_name: &str) -> Self {
        let recipient_name = recipient.get_name();
        let inviter_name = inviter.get_name();

        MailTemplate {
            subject: format!("{inviter_name} invited you to join {team_name}"),
            content: format!(
                r#"
                <h1>Hello <b>{recipient_name}</b>,</h1>
                <p>You're invited by <b>{inviter_name}</b> to join the <b>{team_name}</b> team!</p>
                <p>Click on the button below to confirm your invite.</p>
                <button>Join now</button>
            "#
            ),
        }
    }
}

//impl MailBuilder {
//    pub fn join_team(&self, recipient: &PublicUser, inviter: &PublicUser, team_name: &str) {
//        let recipient_name = recipient.get_name();
//        let inviter_name = inviter.get_name();
//
//        let cid = "<inline-image>".to_string(); 
//
//        let content = format!(
//            r#"
//            <h1>Hello <b>{recipient_name}</b>,</h1>
//            <p>You're invited by <b>{inviter_name}</b> to join the <b>{team_name}</b> team!</p>
//            <p>Click on the button below to confirm your invite.</p>
//            <button>Join now</button>
//            <img src=\"cid:{}\" alt=\"Project Image\">
//        "#,
//        cid
//        );
//
//        let image_path = "path/to/image";
//
//        self.build(recipient)
//            .subject(format!("{inviter_name} invited you to join {team_name}"))
//            .multipart(
//                MultiPart::related() // Required for inline images
//                .singlepart(SinglePart::html(content))
//                .singlepart(
//                    Attachment::new_inline("image.png".to_string())
//                    .body(std::fs::read(image_path).unwrap(), "image/png".parse().unwrap())
//                    .set_content_id(cid),
//                    //.content_id(cid),
//
//                ),
//            )
//            .map_err(|e| e.to_string())?;
//    }
//}


// 1️⃣ Inline CID Attachment (Recommended)
// This method attaches the image inside the email and references it with a cid (Content-ID).
// 
// 📌 Steps to Embed Local Image
// Read the image from your project
// 
// Attach it to the email
// 
// Reference it using cid:image_id in the HTML
// 
// Here’s how to do it in Rust with lettre:
// 
// rust
// Copy
// Edit
// use std::fs;
// use lettre::message::{SinglePart, Attachment, MultiPart};
// use lettre::Message;
// 
// fn build_email_with_local_image(recipient_email: &str, image_path: &str) -> Message {
//     let cid = "<inline-image>".to_string(); // Unique identifier for the image
// 
//     let html_body = format!(
//         "<html>
//             <body>
//                 <h1>Hello,</h1>
//                 <p>Here is an inline image from our project:</p>
//                 <img src=\"cid:{}\" alt=\"Project Image\">
//             </body>
//         </html>",
//         cid
//     );
// 
//     Message::builder()
//         .from("Rustle <noreply@rustle.com>".parse().unwrap())
//         .to(recipient_email.parse().unwrap())
//         .subject("Email with Local Image")
//         .multipart(
//             MultiPart::related()
//                 .singlepart(SinglePart::html(html_body))
//                 .singlepart(
//                     Attachment::new_inline("image.png".to_string()) // The attachment name
//                         .body(fs::read(image_path).unwrap(), "image/png".parse().unwrap())
//                         .content_id(cid),
//                 ),
//         )
//         .unwrap()
// }
// ✅ Pros
// ✔️ Works even if the recipient is offline
// ✔️ Ensures the image is always displayed
// 
// ❌ Cons
// ❌ Increases email size
// ❌ Some email clients block inline attachments