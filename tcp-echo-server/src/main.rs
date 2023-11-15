use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8, 255];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap_or_default();
            true
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap_or_default();
            false
        }
    } {}
}

fn main() {
    let addr = "0.0.0.0:33333";
    let listener = TcpListener::bind(addr).unwrap();
    println!("server listen on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
