use std::net::TcpListener;
use crate::net::{
    Error,
    Result,
    Connection
};

///Wrapper over TcpListener from std
///
///Created to reduce the code in
///server.rs and make it reusable
pub struct Listener {
    listener: TcpListener
}

impl Listener {
    pub fn new() -> Result<Self> {
        match TcpListener::bind("127.0.0.1:7080") {
            Ok(listener) => Ok( Self{ listener } ),
            Err(_) => return Err(Error::ConnectionError)
        }
    }

    pub fn listen(&self) -> ConnectionIter {
        ConnectionIter::new(self.listener.incoming())
    }
}


///Iterator over incoming connections
///
///Returns **ONLY** successful connections wrapped in Connection
pub struct ConnectionIter<'a> {
    iter: std::net::Incoming<'a>
}

impl<'a> ConnectionIter<'a> {
    fn new(iter: std::net::Incoming<'a>) -> Self {
        Self {
            iter
        }
    }
}

impl Iterator for ConnectionIter<'_> {
    type Item = Connection;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;

        if let Ok(next) = next {
            Some(Connection::new(next))
        } else {
            self.next()
        }
    }
}
