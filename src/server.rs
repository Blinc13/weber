//TODO: Refactor this

use crate::parser::request::parser::RequestParser as Parser;

use buffer::ReadBuffer;
use std::net::{TcpListener, TcpStream};
use std::{collections::HashMap, sync::Arc};
use threads_pool::ThreadPool;

type Pages = HashMap<String, Box<dyn Fn() -> String + Sync + Send>>;

///# HttpServer struct.
///
///This is the actual http server.
///Just instantiate and add pages and resources
///
///# Example
///```
///let mut server = weber::HttpServer::new(1);
///
///server.add_page("/".to_string(), || {
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
        T: Fn() -> String + Sync + Send + 'static,
    {
        let func = Box::new(func);

        self.pages.as_mut().unwrap().insert(page, func);
    }

    pub fn run(mut self) {
        let listener = TcpListener::bind("127.0.0.1:7080").unwrap();

        let ptr = Arc::new(self.pages.take().unwrap());

        for connection in listener.incoming() {
            if let Ok(connection) = connection {
                self.listen_connection(connection, ptr.clone());
            }
        }
    }

    fn listen_connection(&mut self, connection: TcpStream, ptr: Arc<Pages>) {
        self.workers
            .execute(move || {
                Self::response(connection, ptr);
            })
            .unwrap();
    }

    fn response(mut stream: TcpStream, pages_list: Arc<Pages>) {
        let buf = Self::read_from_stream(&mut stream, 512);
        let parsed_header = Parser::parse(&buf).unwrap();

        let func = pages_list.get(parsed_header.path);

        let content = match func {
            Some(func) => func(),
            None => "Page not found".to_string(),
        };
    }

    fn read_from_stream(stream: &mut TcpStream, len: usize) -> String {
        let mut buf = Vec::with_capacity(len);
        stream.read_buffer(&mut buf).unwrap();

        String::from_utf8(buf).unwrap()
    }
}
