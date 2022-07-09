use std::net::TcpListener;
use crate::net::Connection;

pub struct Listener {
    listener: TcpListener
}

impl Listener {
    pub fn new() -> Self {
        Self {
            listener: TcpListener::bind("127.0.0.1:7080").unwrap()
        }
    }

    pub fn listen(&self) -> ConnectionIter {
        ConnectionIter::new(self.listener.incoming())
    }
}



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
