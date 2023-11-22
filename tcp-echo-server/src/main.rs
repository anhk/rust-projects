use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 33333)]
    port: u16,
}

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
    let args = Args::parse();
    let addr = format!(":::{}",args.port);
    let listener = TcpListener::bind(&addr).unwrap();
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
