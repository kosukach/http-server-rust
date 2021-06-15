use std::io::{Read, Write, BufWriter, BufReader, Result};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    let mut buffer = String::new();
    let mut reader = BufReader::new(&stream);
    match reader.read_to_string(&mut buffer) {
        Ok(buff) => println!("{}, {}", buffer, buff),
        Err(_) => (),
    }

    let res = b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello world!";
    let mut writer = BufWriter::new(&stream);
    match writer.write(res) {
       Ok(buff) => println!("{}", buff),
       Err(_) => (),
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
