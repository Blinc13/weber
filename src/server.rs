//!This module contains server structure

use crate::{
    net::{
        Listener,
        Connection
    },
    parser::{
        Content,
        ContentType,
        request::parser::RequestParser,
        response::builder::ResponseBuilder
    }
};

use std::{collections::HashMap, sync::Arc, io::Read};
use threadpool::ThreadPool;

type Page = Box<dyn Fn(&RequestParser) -> Content + Sync + Send>;
type Pages = HashMap<String, Page>;

///# HttpServer struct.
///
///This is the actual http server.
///Just instantiate and add pages and resources
///
///# Example
///```
///use weber::parser::{Content, ContentType};
///let mut server = weber::HttpServer::new(1);
///
///server.add_page("/", | _ | {
///    Content::new("Some html!".as_bytes().to_vec(), ContentType::Html, 200)
///});
///```
pub struct HttpServer {
    workers: ThreadPool,
    pages: Option<Pages>,
    notfound_handler: Page
}

impl HttpServer {
    pub fn new(thread_count: usize) -> Self {
        let workers = ThreadPool::new(thread_count);
        let pages = Some(HashMap::new());

        Self {
            workers,
            pages,
            notfound_handler: Box::new(standart_notfound_handler)
        }
    }

    ///Adds a page handler, see the [**README.md**](https://github.com/Blinc13/weber/blob/master/README.md) for an example
    pub fn add_page<T>(&mut self, page: &str, func: T)
    where T: Fn(&RequestParser) -> Content + Sync + Send + 'static,
    {
        let func = Box::new(func);

        self.pages.as_mut().unwrap().insert(page.to_string(), func);
    }

    ///Sets an invalid url handler.
    ///
    ///# When is it called?
    ///> When there is no page handler
    pub fn set_notfound_handler<T>(&mut self, func: T)
    where T: Fn(&RequestParser) -> Content + Sync + Send + 'static
    {
        self.notfound_handler = Box::new(func);
    }

    ///Adds a page with the specified resource
    ///
    ///# Example
    ///```
    ///use weber::{
    ///    HttpServer,
    ///    parser::ContentType
    ///};
    ///
    ///let mut server = HttpServer::new(1);
    ///
    ///server.add_resource("/favicon.ico", "some_image", ContentType::Ico);
    ///```
    pub fn add_resource(&mut self, page: &str, resource: &'static str, r#type: ContentType) {
        self.add_page(page, move | _ | {
            let mut file = std::fs::File::open(resource).unwrap();
            let mut content= Vec::new();

            file.read_to_end(&mut content).unwrap();

            Content::new(content, r#type, 200)
        });
    }

    ///Starts the server on the given ip and port
    ///
    ///See [**README.md**](https://github.com/Blinc13/weber/blob/master/README.md) for an example.
    pub fn run(mut self, ip: &str) {
        let listener = Listener::new(ip).unwrap();

        let pages = Arc::new(self.pages.take().unwrap());
        let handler = Arc::new(self.notfound_handler);

        for connection in listener.listen() {
            let pages = pages.clone();
            let handler = handler.clone();

            self.workers.execute(move || {
                Self::response(connection, pages, handler)
            });
        }
    }

    fn response(mut connection: Connection, pages_list: Arc<Pages>, notfound_handler: Arc<Page>) {
        let parsed = connection.parse_incoming().unwrap().as_request();
        let parsed_path = &parsed.path;

        let content = match pages_list.get(&parsed_path.path) {
            Some(page) => page(&parsed),
            None => notfound_handler(&parsed)
        };

        connection.write_builder(
            ResponseBuilder::new()
                .set_code(content.status_code)
                .set_reason(&content.reason)
                .set_content(&content.content, content.r#type)
        ).unwrap();
    }
}

fn standart_notfound_handler(_parsed: &RequestParser) -> Content {
    let html = "<!DOCTYPE html>
    <html><head><h1>Weber</h1></head><body>404 Not Found</body></html>";

    Content::new_with_reason(html.as_bytes().to_vec(), ContentType::Html, 404, "PAGE NOT FOUND")
}