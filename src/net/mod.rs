//!This module contains methods to simplify networking

pub use connection::Connection;
pub use listener::Listener;
pub use httpdata::HttpData;

pub mod connection;
pub mod listener;
mod httpdata;

pub type Result<T> = std::result::Result<T, Error>;

///This enum contains a network related error.
///
///# Possible states
///- *ConnectionLost*
///     > The other side dropped the connection
///- *ConnectionError*
///- *BadRequest*
///     > The second party sent a request / response that cannot be parsed
///- *UnableToRead*
///     > Ability to read request lost
#[derive(Debug)]
pub enum Error {
    ConnectionLost,
    ConnectionError,
    BadRequest,
    UnableToRead
}