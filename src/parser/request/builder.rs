use crate::parser::Header;
use std::{
    io::{Result, Write},
    net::TcpStream,
};

pub const GET: &str = "GET";
pub const POST: &str = "POST";


///This structure describes the request builder
///
///It's easier to see than to describe
///# Example
///```
///use weber::parser::request::builder::{GET, RequestBuilder};
///
///let builded_string = RequestBuilder::new()
///        .set_version(1)
///        .set_method(GET)
///        .set_path("/foo/bar")
///        .set_header("Test", "Example")
///        .build();
///
///println!("{}", builded_string);
///```
///# PS
///All structure fields are public and can be set manually
pub struct RequestBuilder<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: u8,

    pub headers: Vec<Header<'a>>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            method: GET,
            path: "/",
            version: 1,

            headers: Vec::new(),
        }
    }

    pub fn set_method(mut self, method: &'a str) -> Self {
        self.method = method;

        self
    }

    pub fn set_path(mut self, path: &'a str) -> Self {
        self.path = path;

        self
    }

    pub fn set_version(mut self, version: u8) -> Self {
        self.version = version;

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

    ///Consumes a builder and writes it to the passed stream
    pub fn send(self, stream: &mut TcpStream) -> Result<usize> {
        stream.write(self.build().as_bytes())
    }

    fn format(&self) -> String {
        let headers: String = self
            .headers
            .iter()
            .map(|header| header.to_string() + "\n")
            .collect();

        format!(
            "{} {} HTTP/1.{}\r\n{}",
            self.method, self.path, self.version, headers
        )
    }
}
