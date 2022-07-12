//!This module contains parsers and builders structures

pub use header::Header;
pub use builder::Builder;
pub use content::ContentType;

pub mod request;
pub mod response;
mod header;
mod builder;
mod content;

pub type Result<T> = std::result::Result<T, Error>;

///Describes an error that occurred during parsing
///
///# Possible States
///- *InvalidFormat*
///     > If *InvalidFormat*, then the content is not as expected<br>
///       and the structure is not suitable for parsing it
///- *StatusCode*
///     > If *StatusCode*, perhaps the number was<br>
///       written incorrectly and could not be parsed
///- *Version*
///     > This protocol version is not supported<br>
///       or it was written incorrectly
///- *Reason*
///- *Method*
///     > Invalid method !
///- *Path*
///     > Error in the patch parsing
#[derive(Debug)]
pub enum Error {
    InvalidFormat,
    StatusCode,
    Version,
    Reason,
    Method,
    Path
}