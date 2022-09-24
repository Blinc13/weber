[<img src="https://crates.io/favicon.ico" width="50"/>](https://crates.io/crates/weber)
[<img src="https://github.githubassets.com/favicons/favicon.svg" width="50"/>](https://github.com/Blinc13/weber)
---
# Example
```rust
use std::collections::HashMap;
use std::net::{SocketAddr, SocketAddrV4};
use weber::*;

fn main() {
    // Building a server that runs in 4 threads
    let mut server = HttpServer::new(4);

    // Creating map with default values
    let mut default = HashMap::new();

    default.insert("foo".to_string(), "gg".to_string());
    default.insert("bar".to_string(), "hi".to_string());

    server.add_page("/", move | request | {
        let vals = request.path.values.as_ref().unwrap_or(&default);

        // Getting values from url
        let foo = match vals.get("foo") {
            None => "Variable not defined",
            Some(foo) => foo
        };

        let bar = match vals.get("bar") {
            None => "Variable not defined",
            Some(bar) => bar
        };

        let html = format!(
            "<!DOCTYPE html>
            <html>
                <head>
                    <h1>Hello world!</h1>
                </head>
                <body>
                    <h3>Foo: {}</h3>
                    <h3>Bar: {}</h3>

                    <a href=\"/:foo=bar,bar=foo\">Some page</a>
                </body>
            </html>
        ", foo, bar); // Format html using received values

        Content::new(html.as_bytes().to_vec(), ContentType::Html, 200)
    });

    let addr = SocketAddrV4::new(
        "127.0.0.1".parse().unwrap(),
        7080
    );

    server.run(SocketAddr::V4(addr)); // Run server on localhost:7080
}
```
---
# Functional
**This package contains the following:**
- *Request* / *response* builders and parsers
> There is also PathParser, but there is no builder for it
- Multithreaded HTTP Server
- And some more functionality

## PS
I hope I was able to interest you *:)*