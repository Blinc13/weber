use std::fmt::{Formatter, Display};
use ContentType::*;

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