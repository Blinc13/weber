pub use header::Header;
pub use builder::Builder;

pub mod request;
pub mod response;
mod header;
mod builder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    StatusCode,
    Version,
    Reason,
    Method,
    Path
}