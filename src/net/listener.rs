use std::net::{TcpListener, Incoming};
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

    pub fn listen(&self) {
        let iter = ConnectionIter::new(self.listener.incoming());

        let vec: Vec<_> = iter.take(1).collect();

        println!("{:?}", vec);
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
