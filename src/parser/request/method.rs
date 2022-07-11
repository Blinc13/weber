use Method::*;
use std::{
    fmt,
    str::FromStr,
    cmp::PartialEq
};


///This struct describes request method
///
///# Possible states
///- GET
///- POST
///# Remark
///1. Structure can be parsed from string
///2. Structure can be displayed on standard output
#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = if let GET = self {
            "GET"
        } else {
            "POST"
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "GET" {
            Ok(GET)
        } else if s == "POST" {
            Ok(POST)
        } else {
            Err(())
        }
    }
}