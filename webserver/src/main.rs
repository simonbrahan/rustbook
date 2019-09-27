use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread_handle: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} running", id);

                job.call_box();
            }),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// `max_workers` is the maximum number of active workers in the ThreadPool
    ///
    /// # Panics
    ///
    /// Will panic if `max_workers` is zero.
    fn new(max_workers: usize) -> ThreadPool {
        assert!(max_workers > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(max_workers);

        for id in 0..max_workers {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, process: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(process);
        self.sender.send(job).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Clippy suggests read_exact here: however, read_exact always reads enough to fill the buffer.
    // A request that is smaller than 512 bytes will hang the server
    // since the stream is never large enough to fill the buffer.
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep";

    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 Not Found", "404.html")
    };

    let response = build_response(status, filename);

    stream.write_all(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}

fn build_response(status: &str, filename: &str) -> String {
    let headers = vec![
        format!("HTTP/1.1 {}", status),
        "Content-Type: text/html; charset=utf-8".to_string(),
    ];

    let head_string = headers.join("\r\n");

    let contents = fs::read_to_string(format!("webpages/{}", filename)).unwrap();

    format!("{}\r\n\r\n{}", head_string, contents).to_string()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
