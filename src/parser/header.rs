use std::fmt::Display;

///Describes header
///
///Used in builders, not to use a lot of memory
pub struct Header<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> Header<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Header { key, value }
    }
}

impl<'a> Display for Header<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}
