use std::fmt::{Formatter, Display};
use ContentType::*;

pub struct Content {
    pub content: Vec<u8>,
    pub r#type: ContentType
}

impl Content {
    pub fn new(content: Vec<u8>, r#type: ContentType) -> Self {
        Self {
            content,
            r#type
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    Html,
    Json,
    Png,
    Jpeg
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Html => "text/html",
            Json => "text/json",
            Png => "text/png",
            Jpeg => "text/jpeg",
        };

        write!(f, "{}", s)
    }
}