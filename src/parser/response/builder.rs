use crate::parser::Header;
use std::{
    io::{Result, Write},
    net::TcpStream,
};

pub struct ResponseBuilder<'a> {
    pub version: u8,
    pub code: u16,

    pub reason: &'a str,
    pub headers: Vec<Header<'a>>,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new() -> Self {
        Self {
            version: 1,
            code: 200,

            reason: "OK",
            headers: Vec::new(),
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

    pub fn set_header(mut self, key: &'a str, value: &'a str) -> Self {
        let header = Header::new(key, value);
        self.headers.push(header);

        self
    }

    pub fn build(self) -> String {
        self.format()
    }

    pub fn send(self, stream: &mut TcpStream) -> Result<usize> {
        stream.write(self.build().as_bytes())
    }

    fn format(&self) -> String {
        let header: String = self
            .headers
            .iter()
            .map(|header| header.to_string() + "\n")
            .collect();

        format!(
            "HTTP/1.{} {} {}\r\n{}\r\n",
            self.version, self.code, self.reason, header
        )
    }
}
