use std::collections::HashMap;
use weber::HttpServer;

fn main() {
    let mut server = HttpServer::new(4);

    let mut default = HashMap::new();

    default.insert("foo".to_string(), "gg".to_string());
    default.insert("bar".to_string(), "hi".to_string());

    server.add_page("/".to_string(),move | request | {
        let vals = request.path.values.as_ref().unwrap_or(&default);

        let foo = match vals.get("foo") {
            None => "Variable not defined",
            Some(foo) => foo
        };

        let bar = match vals.get("bar") {
            None => "Variable not defined",
            Some(bar) => bar
        };

        format!("<!DOCTYPE html>
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
        ", foo, bar)
    });

    server.run("127.0.0.1:7080");
}
