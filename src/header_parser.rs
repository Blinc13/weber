//TODO:
//Rewrite this shit!
use std::collections::HashMap;
use http::request;
use httparse::{
    EMPTY_HEADER,
    Request
};



pub struct Parser<'a> {
    method: &'a str,
    version: u8,
    path: &'a str,

    headers: HashMap<&'a str, &'a [u8]>
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        let mut buf = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut buf);

        request.parse(content.as_bytes()).unwrap();


        let method = request.method.unwrap();
        let version = request.version.unwrap();
        let path = request.path.unwrap();

        let mut headers = HashMap::new();

        for header in buf.iter() {
            headers.insert(
                header.name,
                header.value
            );
        }

        Parser {
            method,
            version,
            path,

            headers
        }
    }

    pub fn build_request(self) -> request::Request<()> {
        let mut req = request::Builder::new()
            .method(self.method)
            .uri(self.path);    //Creating request witch patch and method

        for (k, v) in self.headers {
            req = req.header(k, v);
        } //Adding headers to request

        req.body(()).unwrap()
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn headers(&self) -> &HashMap<&'a str, &'a [u8]> {
        &self.headers
    }
}