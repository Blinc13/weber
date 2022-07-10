//TODO: Add normal error handling
use buffer::ReadBuffer;
use std::io::{
    Read,
    Write,
    Result,
    BufRead,
    BufReader
};
use std::net::TcpStream;
use crate::parser::request::parser::RequestParser;

const CAPACITY: usize = 512;

pub struct Connection {
    stream: Option<TcpStream>,

    readed: bool
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            readed: false
        }
    }

    pub fn parse_incoming(&mut self) -> Option<RequestParser> {
        if self.readed {
            return None;
        }

        let mut buf = BufReader::new(self.stream.take().unwrap());
        let mut vec = Vec::with_capacity(CAPACITY);

        while let Ok(n) = buf.read_until('\n' as u8, &mut vec) {
            if n <= 2 {
                break;
            }
        }

        let mut parsed = RequestParser::parse(&vec).unwrap();

        parsed.content = match parsed.headers.get("Content-Length") {
            Some(lenght) => {
                let lenght: usize = String::from_utf8_lossy(lenght).parse().unwrap();

                let mut vec = vec![0u8; lenght];
                buf.read_exact(&mut vec).unwrap();

                Some(vec)
            }

            None => {
                None
            }
        };

        self.stream = Some(buf.into_inner());

        Some(parsed)
    }

    pub fn write_buf(&mut self, buf: & [u8]) -> Result<()> {
        let mut stream = self.stream.take().unwrap();

        stream.write_all(&buf)?;

        self.stream = Some(stream);

        Ok(())
    }
}