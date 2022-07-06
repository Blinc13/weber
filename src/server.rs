use httparse::Result;
use threads_pool::ThreadPool;
use std::collections::HashMap;

pub struct HttpServer {
    workers: ThreadPool,
    pages: HashMap<String, Box<dyn Fn() -> String>>
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

    }
}