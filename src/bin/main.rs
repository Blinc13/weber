use std::collections::HashMap;
use weber::{
    HttpServer,
    parser::{
        Content,
        ContentType
    }
};

fn main() {
    // Building a server that runs in 4 threads
    let mut server = HttpServer::new(4);

    // Creating map with default values
    let mut default = HashMap::new();

    default.insert("foo".to_string(), "gg".to_string());
    default.insert("bar".to_string(), "hi".to_string());

    server.add_page("/".to_string(),move | request | {
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

        Content::new(html.as_bytes().to_vec(), ContentType::Html)
    });

    server.run("127.0.0.1:7080"); // Run server on localhost:7080
}
