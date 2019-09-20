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

    let contents = fs::read_to_string("webpages/hello.html").unwrap();

    let response = build_response(&contents);

    stream.write_all(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}

fn build_response(contents: &str) -> String {
    let headers = vec!["HTTP/1.1 200 OK", "Content-Type: text/html; charset=utf-8"];

    let head_string = headers.join("\r\n");

    format!("{}\r\n\r\n{}", head_string, contents).to_string()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
