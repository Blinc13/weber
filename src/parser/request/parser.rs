use httparse::{Request, EMPTY_HEADER};
use std::collections::HashMap;
use crate::parser::{
    Error,
    request::Method
};

///This structure describes the parsed HTTP request
///
///All structure fields are public, access them directly
///# Example
///```
///use weber::parser::{
///    Builder,
///    request::{
///        parser::RequestParser,
///        builder::RequestBuilder,
///
///        Method::GET
///    },
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

        if let Err(_) = request.parse(content) {
            return Err(Error::InvalidFormat);
        }

        let method_str = match request.method {
            None => return Err(Error::Method),
            Some(i) => i
        };
        let version = match request.version {
            None => return Err(Error::Version),
            Some(i) => i
        };
        let path = match request.path {
            None => return Err(Error::Path),
            Some(i) => i.to_string()
        };

        let mut headers = HashMap::new();
        let method: Method;

        {//Data representation operations
            for header in buf.iter() {
                headers.insert(header.name.to_string(), header.value.to_vec());
            }

            method = match method_str.parse() {
                Ok(i) => i,
                Err(_) => return Err(Error::Method)
            };
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
