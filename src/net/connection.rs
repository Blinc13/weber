use std::io::{Read, Write, BufRead, BufReader, ErrorKind};
use std::net::TcpStream;
use crate::{
    net::{
        Error,
        Result,
        HttpData
    },
    parser::Builder
};

const CAPACITY: usize = 512;

///Simplified connection
///
///Has the most simplified methods for communication
pub struct Connection {
    stream: Option<TcpStream>,

    readed: bool,
    writed: bool
}

impl Connection {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            readed: false,
            writed: false
        }
    }

    ///Makes a connection and returns a Connect if successful
    pub fn connect(ip: &str) -> Result<Connection> {
        let stream = match TcpStream::connect(ip) {
            Ok(stream) => stream,
            Err(_) => return Err(Error::ConnectionError)
        };

        Ok(Self::new(stream))
    }

    ///Performs stream reading and parsing
    ///
    ///Returns a request / response if successful
    pub fn parse_incoming(&mut self) -> Result<HttpData> {
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


        let mut parsed = match HttpData::parse(&vec) {
            Ok(parsed) => parsed,
            Err(_) => return Err(Error::BadRequest)
        }; // Parse the request, if failed, return Err::BadRequest


        let headers = parsed.headers();
        parsed.set_content(match headers.get("Content-Length") {
            Some(lenght) => {
                let lenght: usize = match String::from_utf8_lossy(lenght).parse() {
                    Ok(i) => i,
                    Err(_) => return Err(Error::BadRequest)
                };

                let mut vec = vec![0u8; lenght];

                if let Err(_) = buf.read_exact(&mut vec) {
                    return Err(Error::BadRequest);
                } // If the fields are incorrect, return Error::BadRequest

                Some(vec)
            }

            None => {
                None
            }
        }); // If we have content fields, read the content


        self.stream = Some(buf.into_inner());
        self.readed = true;

        Ok(parsed)
    }

    ///Writes a structure that implements the trait Builder
    ///
    ///Structure in process consumed
    pub fn write_builder<T>(&mut self, response: T) -> Result<()>
        where T: Builder
    {
        if self.writed {
            return Err(Error::UnableToWrite);
        }

        let mut stream = self.stream.take().unwrap();
        let response = response.build(); // Build the response

        if let Err(_) = stream.write_all(&response) {
            return Err(Error::UnableToWrite);
        } // If unable to write, return an error

        if let Err(e) = stream.flush() {
            return match e.kind() {
                ErrorKind::ConnectionAborted => Err(Error::ConnectionLost),
                _ => Err(Error::ConnectionError)
            }
        } // If sending failed, return an error

        self.stream = Some(stream);
        self.writed = true;

        Ok(())
    }
}
