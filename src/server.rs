use buffer::ReadBuffer;
use crate::header_parser::Parser;
use httparse::Result;
use threads_pool::ThreadPool;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

type Pages = HashMap<String, Box<dyn Fn() -> String>>;

///# HttpServer struct.
///
///This is the actual http server.
///Just instantiate and add pages and resources
///
///# Example
///```
///let mut server = weber::HttpServer::new();
///
///server.add_page("/".to_string(), || {
///   "some_page".to_string()
///});
///```
pub struct HttpServer {
    workers: ThreadPool,
    pages: Pages
}

impl HttpServer {
    pub fn new(thread_count: usize) -> Self {
        let workers = ThreadPool::new(thread_count);
        let pages = HashMap::new();

        Self {
            workers,
            pages
        }
    }

    pub fn add_page<T>(&mut self, page: String, func: T)
        where T: Fn() -> String + 'static
    {
        let func = Box::new(func);

        self.pages.insert(page, func);
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7080")
            .unwrap();

        for connection in listener.incoming() {
            self.workers.execute(|| {
                match connection {
                    Ok(stream) => Self::handle_connection(stream),
                    Err(_) => {}
                }
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut header = Vec::with_capacity(512);
        stream.read_buffer(&mut header).unwrap();

        let header = String::from_utf8_lossy(&header);

        let parser = Parser::new(&header);

        let method = parser.method();
        let path = parser.path();
        let version = parser.version();

        let headers = parser.headers();

        println!("Version: {}", version);
        println!("Path: {}", path);
        println!("Method: {}\n\n", method);

        println!("{:?}", headers);
    }
}