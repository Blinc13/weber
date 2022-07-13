//TODO: Refactor this

use crate::parser::{Content, ContentType, request::parser::RequestParser, response::builder::ResponseBuilder};
use crate::net::{
    Listener,
    Connection
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
///    Content::new("Some html!".as_bytes().to_vec(), ContentType::Html)
///});
///```
pub struct HttpServer {
    workers: ThreadPool,
    pages: Option<Pages>,
}

impl HttpServer {
    pub fn new(thread_count: usize) -> Self {
        let workers = ThreadPool::new(thread_count);
        let pages = Some(HashMap::new());

        Self { workers, pages }
    }

    ///Adds a closure associated with the page
    pub fn add_page<T>(&mut self, page: &str, func: T)
    where
        T: Fn(&RequestParser) -> Content + Sync + Send + 'static,
    {
        let func = Box::new(func);

        self.pages.as_mut().unwrap().insert(page.to_string(), func);
    }

    pub fn add_resource(&mut self, page: &str, resource: &'static str, r#type: ContentType) {
        self.add_page(page, move | _ | {
            let mut file = std::fs::File::open(resource).unwrap();
            let mut content= Vec::new();

            file.read_to_end(&mut content).unwrap();

            Content::new(content, r#type)
        });
    }

    pub fn run(mut self, ip: &str) {
        let listener = Listener::new(ip).unwrap();

        let ptr = Arc::new(self.pages.take().unwrap());

        for connection in listener.listen() {
            let copy = ptr.clone();

            self.workers.execute(move || {
                Self::response(connection, copy)
            });
        }
    }

    fn response(mut connection: Connection, pages_list: Arc<Pages>) {
        let parsed = connection.parse_incoming().unwrap().as_request();
        let parsed_path = &parsed.path;

        let content = match pages_list.get(&parsed_path.path) {
            Some(func) => func(&parsed),
            None => Content::new("PAGE NOT FOUND".as_bytes().to_vec(), ContentType::Html)
        };

        let resp = ResponseBuilder::new()
                .set_content(&content.content, content.r#type);

        connection.write_builder(resp).unwrap();
    }
}
