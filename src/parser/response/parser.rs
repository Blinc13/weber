use httparse::{Response, EMPTY_HEADER};
use std::collections::HashMap;
use crate::parser::Error;


///This structure describes the parsed response
///
///The same rights apply here as for
///weber::parser::request::parser::RequestParser
pub struct ResponseParser {
    pub version: u8,
    pub status_code: u16,

    pub reason: Option<String>,
    pub headers: HashMap<String, Vec<u8>>,
    pub content: Option<Vec<u8>>
}

impl ResponseParser {
    pub fn parse(content: &[u8]) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut response = Response::new(&mut buf);

        if let Err(_) = response.parse(content) {
            return Err(Error::InvalidFormat);
        }

        let version = match response.version {
            None => return Err(Error::Version),
            Some(i) => i
        };
        let status_code = match response.code {
            None => return Err(Error::StatusCode),
            Some(i) => i
        };
        let reason = response.reason.map(|s| s.to_string());

        let mut headers = HashMap::new();

        for header in buf {
            headers.insert(header.name.to_string(), header.value.to_vec());
        }

        Ok(Self {
            version,
            status_code,
            reason,
            headers,
            content: None
        })
    }
}
