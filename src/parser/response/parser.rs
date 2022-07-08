use std::collections::HashMap;

use httparse::{Error, Response, EMPTY_HEADER};


///This structure describes the parsed response
///
///The same rights apply here as for
///weber::parser::request::parser::RequestParser
pub struct ResponseParser<'a> {
    pub version: u8,
    pub code: u16,

    pub reason: Option<&'a str>,
    pub headers: HashMap<&'a str, &'a [u8]>,
}

impl<'a> ResponseParser<'a> {
    pub fn parse(content: &'a [u8]) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut response = Response::new(&mut buf);

        response.parse(&content)?;

        let version = response.version.unwrap();
        let code = response.code.unwrap();
        let reason = response.reason;

        let mut headers = HashMap::new();

        for header in buf {
            headers.insert(header.name, header.value);
        }

        Ok(Self {
            version,
            code,
            reason,
            headers,
        })
    }
}
