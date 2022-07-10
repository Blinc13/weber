//TODO: Add normal error handling
use std::io::{Read, Write, BufRead, BufReader};
use std::net::TcpStream;
use crate::{
    net::{
        Error,
        Result
    },
    parser::{
        Builder,
        request::parser::RequestParser
    }
};

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

    pub fn connect(ip: &str) -> Result<Connection> {
        let stream = match TcpStream::connect(ip) {
            Ok(stream) => stream,
            Err(_) => return Err(Error::ConnectionError)
        };

        Ok(Self::new(stream))
    }

    pub fn parse_incoming(&mut self) -> Result<RequestParser> {
        if self.readed {
            return Err(Error::UnableToRead);
        } // If stream is already readed, return Err::BadRequest

        // Create buffers. vec has a minimum size to allocate for optimization
        let mut buf = BufReader::new(self.stream.take().unwrap());
        let mut vec = Vec::with_capacity(CAPACITY);

        while let Ok(n) = buf.read_until('\n' as u8, &mut vec) {
            if n <= 2 {
                break;
            }
        } // Read until \r\n

        let mut parsed = match RequestParser::parse(&vec) {
            Ok(parsed) => parsed,
            Err(_) => return Err(Error::BadRequest)
        }; // Parse the request, if failed, return Err::BadRequest

        parsed.content = match parsed.headers.get("Content-Length") {
            Some(lenght) => {
                let lenght: usize = String::from_utf8_lossy(lenght).parse().unwrap();

                let mut vec = vec![0u8; lenght];

                if let Err(_) = buf.read_exact(&mut vec) {
                    return Err(Error::BadRequest);
                } // If the fields are incorrect, return Error::BadRequest

                Some(vec)
            }

            None => {
                None
            }
        }; // If we have content fields, read the content

        self.stream = Some(buf.into_inner());

        Ok(parsed)
    }

    pub fn write_builder<T>(&mut self, response: T) -> Result<()>
        where T: Builder
    {
        let mut stream = self.stream.take().unwrap();
        let response = response.build(); // Build the response

        if let Err(e) = stream.write_all(response.as_bytes()) {
            return match e.kind() {
                std::io::ErrorKind::ConnectionAborted => Err(Error::ConnectionLost),
                _ => Err(Error::ConnectionError)
            }
        } // If unable to send, return an error

        stream.flush().unwrap();

        self.stream = Some(stream);

        Ok(())
    }
}