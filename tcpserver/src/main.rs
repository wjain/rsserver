use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // let result = listener.accept().unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!(
            "Recive from client: {:?}",
            str::from_utf8(&buffer[..bytes_read]).unwrap()
        );

        stream.write(&mut buffer[..bytes_read]).unwrap();
    }
}
