use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    println!("Hello, world!");
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer[..bytes_read]).unwrap()
    );
}
