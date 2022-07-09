use buffer::ReadBuffer;

use std::io::{Result, Write};
use std::net::TcpStream;

use crate::parser::{
    request::parser::RequestParser,
    response::builder::ResponseBuilder
};

const CAPACITY: usize = 512;

pub struct Connection {
    stream: TcpStream,

    readed: bool
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Self {
            stream,
            readed: false
        }
    }

    pub fn read_buf(&mut self) -> Option<Vec<u8>> {
        if self.readed {
            return None;
        }

        let mut buf = Vec::with_capacity(CAPACITY);
        self.stream.read_buffer(&mut buf);

        self.readed = true;
        Some(buf)
    }

    pub fn write_buf(&mut self, buf: & [u8]) -> Result<()> {
        self.stream.write_all(buf)?;

        Ok(())
    }
}