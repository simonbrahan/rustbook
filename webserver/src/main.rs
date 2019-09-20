use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Clippy suggests read_exact here: however, read_exact always reads enough to fill the buffer.
    // A request that is smaller than 512 bytes will hang the server
    // since the stream is never large enough to fill the buffer.
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
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

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
