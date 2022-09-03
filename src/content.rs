use std::fmt::{Formatter, Display};
use ContentType::*;

///Structure for the content returned by the page
///
///From the example in the [**README.md**](https://github.com/Blinc13/weber/blob/master/README.md), you can already understand how to use
pub struct Content {
    pub content: Vec<u8>,
    pub r#type: ContentType,
    pub status_code: u16,
    pub reason: String
}

impl Content {
    pub fn new(content: Vec<u8>, r#type: ContentType, status_code: u16) -> Self {
        Self::new_with_reason(content, r#type, status_code, "OK")
    }

    pub fn new_with_reason(content: Vec<u8>, r#type: ContentType, status_code: u16, reason: &str) -> Self {
        Self {
            content,
            r#type,
            status_code,
            reason: reason.to_string()
        }
    }
}

///Enumeration of content-type header values
///
///Just return what the content is
#[derive(Debug)]
pub enum ContentType {
    Html,
    Json,
    Png,
    Jpeg,
    Ico,
    Javascript,
    Text
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Html => "text/html",
            Json => "text/json",
            Png => "image/png",
            Jpeg => "image/jpeg",
            Ico => "image/x-icon",
            Javascript => "text/javascript",
            Text => "text/plain"
        };

        write!(f, "{}", s)
    }
}

impl Clone for ContentType {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for ContentType {}