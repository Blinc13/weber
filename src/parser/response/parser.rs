use std::collections::HashMap;

use httparse::{Error, Response, EMPTY_HEADER};


///This structure describes the parsed response
///
///The same rights apply here as for
///weber::parser::request::parser::RequestParser
pub struct ResponseParser {
    pub version: u8,
    pub code: u16,

    pub reason: Option<String>,
    pub headers: HashMap<String, Vec<u8>>,
}

impl ResponseParser {
    pub fn parse(content: &[u8]) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut response = Response::new(&mut buf);

        response.parse(&content)?;

        let version = response.version.unwrap();
        let code = response.code.unwrap();
        let reason = response.reason.map(|s| s.to_string());

        let mut headers = HashMap::new();

        for header in buf {
            headers.insert(header.name.to_string(), header.value.to_vec());
        }

        Ok(Self {
            version,
            code,
            reason,
            headers,
        })
    }
}
