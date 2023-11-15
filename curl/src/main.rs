use clap::Parser;
use std::{
    io::{self, BufReader, Read, Write},
    net::TcpStream,
    str,
};

mod url;
use url::Url;

/// curl by rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// target to connect
    target: String,
}

fn main() -> io::Result<()> {
    let args = Arguments::parse();

    let url: Url = Url::new(&args.target);
    println!("{}{}:{}", url.protocol, url.domain, url.port);

    let input = format!(
        "GET /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path, url.domain,
    );

    println!("{}", input);

    let mut stream =
        TcpStream::connect(format!("{}:{}", url.domain, url.port)).expect("connect failed");
    stream.set_nodelay(true).expect("set nodelay failed");
    stream.write_all(input.as_bytes()).expect("write failed");

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("read from stream failed");
    println!("{}", str::from_utf8(&buffer).unwrap());
    Ok(())
}
