use httparse::{Error, Request, EMPTY_HEADER};
use std::collections::HashMap;

pub struct RequestParser<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: u8,

    pub headers: HashMap<&'a str, &'a [u8]>,
}

impl<'a> RequestParser<'a> {
    pub fn parse(content: &'a str) -> Result<Self, Error> {
        let mut buf = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut buf);

        request.parse(content.as_bytes())?;

        let method = request.method.unwrap();
        let version = request.version.unwrap();
        let path = request.path.unwrap();

        let mut headers = HashMap::new();

        for header in buf.iter() {
            headers.insert(header.name, header.value);
        }

        Ok(Self {
            method,
            path,
            version,

            headers,
        })
    }
}
