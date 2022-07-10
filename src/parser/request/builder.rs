use crate::parser::{
    Header,
    request::Method
};
use std::{
    io::{Result as IoRes, Write},
    net::TcpStream,
};

///This structure describes the request builder
///
///It's easier to see than to describe
///# Example
///```
///use weber::parser::request::{
///    builder::RequestBuilder,
///    Method::GET
///};
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
///All structure fields are public,
///except for content field,
///because it cannot be used in a GET request
///```
///use weber::parser::request::builder::RequestBuilder;
///
///let builder = RequestBuilder::new().set_content("abc");
///
///assert!(builder.is_err());
///```
pub struct RequestBuilder<'a> {
    pub method: Method,
    pub path: &'a str,
    pub version: u8,

    pub headers: Vec<Header<'a>>,
    content: Option<&'a str>
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            path: "/",
            version: 1,

            headers: Vec::new(),
            content: None
        }
    }

    pub fn set_method(mut self, method: Method) -> Self {
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

    pub fn set_content(mut self, content: &'a str) -> Result<Self, ()> {
        if let Method::POST = self.method {
            self.content = Some(content);

            Ok(self)
        } else {
            Err(())
        }
    }

    pub fn build(self) -> String {
        self.format()
    }

    ///Consumes a builder and writes it to the passed stream
    pub fn send(self, stream: &mut TcpStream) -> IoRes<usize> {
        stream.write(self.build().as_bytes())
    }

    fn format(&self) -> String {
        let headers: String = self
            .headers
            .iter()
            .map(|header| header.to_string() + "\r\n")
            .collect();

        format!(
            "{} {} HTTP/1.{}\r\n{}\r\n",
            self.method, self.path, self.version, headers
        )
    }
}
