use httparse::{Error, Request, EMPTY_HEADER};
use std::collections::HashMap;
use crate::parser::request::Method;

///This structure describes the parsed HTTP request
///
///All structure fields are public, access them directly
///# Example
///```
///use weber::parser::request::{
///    parser::RequestParser,
///    builder::RequestBuilder,
///
///    Method::GET
///};
///
///let request = RequestBuilder::new()
///        .set_header("Example", "test")
///        .build();
///
///let parsed = RequestParser::parse(request.as_bytes())
///        .unwrap();
///
///assert_eq!(parsed.method, GET);
///assert_eq!(parsed.path, "/");
///```
pub struct RequestParser<'a> {
    pub method: Method,
    pub path: &'a str,
    pub version: u8,

    pub headers: HashMap<&'a str, &'a [u8]>,
}

impl<'a> RequestParser<'a> {
    pub fn parse(content: &'a [u8]) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut buf);

        request.parse(content)?;

        let method_str = request.method.unwrap();
        let version = request.version.unwrap();
        let path = request.path.unwrap();

        let mut headers = HashMap::new();
        let method: Method;

        {//Data representation operations
            for header in buf.iter() {
                headers.insert(header.name, header.value);
            }

            method = method_str.parse().unwrap();
        }

        Ok(Self {
            method,
            path,
            version,

            headers,
        })
    }
}
