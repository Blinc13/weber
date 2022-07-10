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
pub struct RequestParser {
    pub method: Method,
    pub path: String,
    pub version: u8,

    pub headers: HashMap<String, Vec<u8>>,
    pub content: Option<Vec<u8>>
}

impl RequestParser {
    pub fn parse(content: &[u8]) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut buf);

        request.parse(content)?;

        let method_str = request.method.unwrap();
        let version = request.version.unwrap();
        let path = request.path.unwrap().to_string();

        let mut headers = HashMap::new();
        let method: Method;

        {//Data representation operations
            for header in buf.iter() {
                headers.insert(header.name.to_string(), header.value.to_vec());
            }

            method = method_str.parse().unwrap();
        }

        Ok(Self {
            method,
            path,
            version,

            headers,
            content: None
        })
    }
}
