pub use net::Connection;
pub use server::HttpServer;

pub use parser::request;
pub use parser::response;

pub use content::Content;
pub use content::ContentType;

pub mod parser;
pub mod server;
pub mod content;
pub mod net;

///Prints a message to stderr
///
///# Example
///- **Without args:**
///```
///use weber::error;
///
///error!("Foo", "Bar");
///```
///> The output will be:
///>> **[2022-07-14 09:15:21 UTC] Foo: Bar**
///- **With args:**
///```
///use weber::error;
///
///error!("Foo", "Bar - {}", 15);
///```
///> The output will be:
///>> **[2022-07-14 09:15:21 UTC] Foo: Bar - 15**
#[macro_export]
macro_rules! error {
    ($from: literal, $message: literal) => {
        eprintln!("[{}] \x1b[93m{}\x1b[0m: \x1b[31m{}\x1b[0m", chrono::Utc::now().to_string(), $from, $message)
    };

    ($from: literal, $message: literal, $($args: tt),+) => {
        eprintln!("[{}] \x1b[93m{}\x1b[0m: \x1b[31m{}\x1b[0m", chrono::Utc::now().to_string(), $from, format!($message, $($args),+))
    };
}

///Prints a message to stdout
///
///# Example
///- **Without args:**
///```
///use weber::message;
///
///message!("Foo", "Bar");
///```
///> The output will be:
///>> **[2022-07-14 09:15:21 UTC] Foo: Bar**
///- **With args:**
///```
///use weber::message;
///
///message!("Foo", "Bar - {}", 15);
///```
///> The output will be:
///>> **[2022-07-14 09:15:21 UTC] Foo: Bar - 15**
#[macro_export]
macro_rules! message {
    ($from: literal, $message: literal) => {
        println!("[{}] {}: {}", chrono::Utc::now().to_string(), $from, $message)
    };
    ($from: literal, $message: literal, $($args: tt),+) => {
        println!("[{}] {}: {}", chrono::Utc::now().to_string(), $from, format!($message, $($args),+))
    };
}
