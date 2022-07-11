use std::collections::HashMap;
use crate::parser::{Result, Error};

///This structure describes the parsed path
///
///All fields are public, as in the rest of the parsers structures
///# Path format
///### The path is written as **/foo/bar:val=true**<br>
///* '**:**' - separator, it separates path from variables
///
///### Variables syntax: **false=true,true=false**<br>
///* '**=**' - separates name and value<br>
///* '**,**' - separates variables
///
///
///***Simple, isn't it?***
///# Example
///```
///use weber::parser::request::PathParser;
///
///let path = "/path/to/foo:name=test,test=name";
///
///let parsed = PathParser::parse(&path).unwrap();
///let vals = parsed.values.as_ref().unwrap();
///
///
///assert_eq!(vals["name"], "test");
///assert_eq!(vals["test"], "name");
///assert_eq!(parsed.path, "/path/to/foo");
///```
pub struct PathParser {
    pub path: String,
    pub values: Option<HashMap<String, String>>
}

impl PathParser {
    pub fn parse(path: &str) -> Result<Self> {
        if path.len() == 0 {
            return Err(Error::InvalidFormat);
        }

        let parts: Vec<_> = path.split(':').collect();

        let path = parts[0].to_string();
        let values = match parts.get(1) {
            Some(s) => {
                let values: Vec<_> = s.split(',').collect();
                let mut map = HashMap::new();

                for val in values.iter() {
                    let parts: Vec<_> = val.split('=').collect();

                    if parts.len() != 2 {
                        return Err(Error::InvalidFormat);
                    }

                    map.insert(parts[0].to_string(), parts[1].to_string());
                }

                //I really wanted to implement this on iterators
                //but I don't know how to handle errors here

                //Oh, and yes, this code parse values in path

                Some(map)
            }

            None => None
        };

        Ok(Self { path, values })
    }
}