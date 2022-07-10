pub use connection::Connection;
pub use listener::Listener;

pub mod connection;
pub mod listener;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectionLost,
    ConnectionError,
    BadRequest,
    UnableToRead
}