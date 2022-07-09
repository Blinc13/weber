//TODO: Refactor this

use crate::parser::{
    request::parser::RequestParser,
    response::builder::ResponseBuilder
};
use crate::net::connection::Connection;

use std::net::TcpListener;
use std::{collections::HashMap, sync::Arc};
use threadpool::ThreadPool;

type Pages = HashMap<String, Box<dyn Fn(RequestParser) -> String + Sync + Send>>;

///# HttpServer struct.
///
///This is the actual http server.
///Just instantiate and add pages and resources
///
///# Example
///```
///let mut server = weber::HttpServer::new(1);
///
///server.add_page("/".to_string(), | _ | {
///   "some_page".to_string()
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
    pub fn add_page<T>(&mut self, page: String, func: T)
    where
        T: Fn(RequestParser) -> String + Sync + Send + 'static,
    {
        let func = Box::new(func);

        self.pages.as_mut().unwrap().insert(page, func);
    }

    pub fn run(mut self) {
        let listener = TcpListener::bind("127.0.0.1:7080").unwrap();

        let ptr = Arc::new(self.pages.take().unwrap());

        for connection in listener.incoming() {
            if let Ok(connection) = connection {
                self.listen_connection(Connection::new(connection), ptr.clone());
            } else {
                println!("Connection error");
            }
        }
    }

    fn listen_connection(&mut self, connection: Connection, ptr: Arc<Pages>) {
        self.workers.execute(move || {
                Self::response(connection, ptr);
            });
    }

    fn response(mut connection: Connection, pages_list: Arc<Pages>) {
        let buf = connection.read_buf().unwrap();

        let parsed = RequestParser::parse(&buf)
                .unwrap();

        let content = match pages_list.get(parsed.path) {
            Some(func) => func(parsed),
            None => "PAGE NOT FOUND".to_string()
        };

        let resp = ResponseBuilder::new()
                .set_content(&content)
                .build();

        connection.write_buf(resp.as_bytes());
    }
}
