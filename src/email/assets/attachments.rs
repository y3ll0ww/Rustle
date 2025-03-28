use lettre::message::{
    header::{ContentType, ContentTypeErr},
    Attachment, SinglePart,
};

use super::Asset;

const CID_LOGO_SMALL: &str = "Rustle";

pub struct InlineAttachment {
    pub html: String,
    pub source: SinglePart,
}

impl Asset for InlineAttachment {}

impl InlineAttachment {
    pub fn logo_small() -> Result<Self, String> {
        Ok(InlineAttachment {
            html: format!("<img src=\"cid:{CID_LOGO_SMALL}\" width=\"150\">"),
            source: Self::singlepart(CID_LOGO_SMALL, FileType::Png("216478030.png"))?,
        })
    }

    fn singlepart(cid: &str, file: FileType) -> Result<SinglePart, String> {
        let content = Self::read_attachment(file.name())?;
        Ok(Attachment::new_inline(cid.to_string()).body(content, file.content_type()?))
    }
}

pub enum FileType<'a> {
    Png(&'a str),
    Jpg(&'a str),
}

impl<'a> FileType<'a> {
    fn name(&self) -> &str {
        match self {
            FileType::Png(val) | FileType::Jpg(val) => val,
        }
    }

    fn content_type(&self) -> Result<ContentType, String> {
        match self {
            FileType::Png(_) => "image/png".parse(),
            FileType::Jpg(_) => "image/jpeg".parse(),
        }
        .map_err(|e: ContentTypeErr| e.to_string())
    }
}
