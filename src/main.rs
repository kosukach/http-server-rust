use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let inhalt = fs::read_to_string("hello.html").unwrap();
    let response = format! {
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        inhalt.len(),
        inhalt
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connected to client");
        handle_client(stream);
    }
}
