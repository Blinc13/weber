use std::collections::HashMap;
use crate::parser::{
    Error,
    Result,
    request::parser::RequestParser,
    response::parser::ResponseParser
};

///Enum contains either a request / or a response
///
///> **Created to be able to return not only a request,
///    but also a response from a Connection::Parse_incoming**
///
///# Remark
///The methods in this enum are **VERY unoptimized**
pub enum HttpData {
    Request(RequestParser),
    Response(ResponseParser)
}

impl HttpData {
    pub fn parse(content: &[u8]) -> Result<Self> {
        if let Ok(parsed) = RequestParser::parse(content) {
            Ok( HttpData::Request(parsed) )
        } else if let Ok(parsed) = ResponseParser::parse(content) {
            Ok( HttpData::Response(parsed) )
        } else {
            Err(Error::InvalidFormat)
        }
    }

    pub fn headers(&self) -> &HashMap<String, Vec<u8>> {
        match &self {
            HttpData::Request(parsed) => &parsed.headers,
            HttpData::Response(parsed) => &parsed.headers
        }
    }

    pub fn content(&self) -> Option<&Vec<u8>> {
        match &self {
            HttpData::Request(parsed) => parsed.content.as_ref(),
            HttpData::Response(parsed) => parsed.content.as_ref()
        }
    }

    pub fn version(&self) -> u8 {
        match &self {
            HttpData::Request(parsed) => parsed.version,
            HttpData::Response(parsed) => parsed.version
        }
    }

    pub fn as_request(self) -> RequestParser {
        match self {
            Self::Request(parsed) => parsed,
            _ => panic!("This is not a request")
        }
    }

    pub fn as_response(self) -> ResponseParser {
        match self {
            Self::Response(parsed) => parsed,
            _ => panic!("This is not a response")
        }
    }
}