use weber::HttpServer;

fn main() {
    let mut server = HttpServer::new(4);

    server.add_page("/".to_string(), || {
        return "<!DOCTYPE html>
        <html>
            <head>
                <h1>Hello world!</h1>
            </head>
        </html>
        ".to_string()
    });

    server.run();
}