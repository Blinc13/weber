//TODO: If possible, reconsider the solution with the content field

use crate::parser::{
    Header,
    Builder
};

///This structure describes the request builder
///
///I don't want to write documentation so see
///weber::parser::request::builder::RequestBuilder
pub struct ResponseBuilder<'a> {
    pub version: u8,
    pub code: u16,

    pub reason: &'a str,
    pub headers: Vec<Header<'a>>,
    pub content: &'a str
}

impl<'a> ResponseBuilder<'a> {
    pub fn new() -> Self {
        Self {
            version: 1,
            code: 200,

            reason: "OK",
            headers: Vec::new(),
            content: ""
        }
    }

    pub fn set_code(mut self, code: u16) -> Self {
        self.code = code;

        self
    }

    pub fn set_version(mut self, version: u8) -> Self {
        self.version = version;

        self
    }

    pub fn set_reason(mut self, reason: &'a str) -> Self {
        self.reason = reason;

        self
    }

    pub fn set_content(mut self, content: &'a str) -> Self {
        self.content = content;

        self
    }

    pub fn set_header(mut self, key: &'a str, value: &'a str) -> Self {
        let header = Header::new(key, value);
        self.headers.push(header);

        self
    }

    fn format(&self) -> String {
        let header: String = self
            .headers
            .iter()
            .map(|header| header.to_string() + "\n")
            .collect();

        format!(
            "HTTP/1.{} {} {}\r\n{}\r\n{}\r\n",
            self.version, self.code, self.reason, header, self.content
        )
    }
}

impl Builder for ResponseBuilder<'_> {
    fn build(self) -> String {
        self.format()
    }
}