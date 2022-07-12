use crate::parser::{
    Header,
    Builder
};

///This structure describes the request builder
///
///I don't want to write documentation so see
///weber::parser::request::builder::RequestBuilder
pub struct ResponseBuilder<'a> {
    pub version: u8,
    pub code: u16,

    pub reason: &'a str,
    pub headers: Vec<Header<'a>>,
    content: Option<&'a [u8]>
}

impl<'a> ResponseBuilder<'a> {
    pub fn new() -> Self {
        Self {
            version: 1,
            code: 200,

            reason: "OK",
            headers: Vec::new(),
            content: None
        }
    }

    pub fn set_code(mut self, code: u16) -> Self {
        self.code = code;

        self
    }

    pub fn set_version(mut self, version: u8) -> Self {
        self.version = version;

        self
    }

    pub fn set_reason(mut self, reason: &'a str) -> Self {
        self.reason = reason;

        self
    }

    pub fn set_content(mut self, content: &'a [u8], content_type: &'a str) -> Self {
        self = self
            .set_header("Content-Type", content_type);

        self.content = Some(content);

        self
    }

    pub fn set_header(mut self, key: &'a str, value: &'a str) -> Self {
        let header = Header::new(key, value);
        self.headers.push(header);

        self
    }

    fn format(&self) -> String {
        let mut header: String = self.headers
            .iter()
            .map(|header| header.to_string() + "\r\n")
            .collect();

        match self.content {
            Some(content) => {
                let len = content.len().to_string();

                header = header +
                    &Header::new("Content-Length", &len).to_string()
                    + "\r\n";
            }
            None => {}
        }

        format!(
            "HTTP/1.{} {} {}\r\n{}\r\n",
            self.version, self.code, self.reason, header
        )
    }
}

impl Builder for ResponseBuilder<'_> {
    fn build(self) -> Vec<u8> {
        let mut res = self.format().as_bytes().to_vec();

        if let Some(content) = self.content {
            res.append(&mut content.to_vec());
        }

        res
    }
}