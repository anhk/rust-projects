use std::{
    env,
    io::{self, BufReader, Read, Write},
    net::TcpStream,
    process::exit,
    str,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("invalid argument.");
        exit(1);
    }

    let addr = &args[1];
    let mut stream = TcpStream::connect(addr)?;

    stream.set_nodelay(true).expect("set nodelay failed");

    let input: String =
        String::from("GET / HTTP/1.1\r\nHost: 2.3.4.5\r\nConnection: close\r\n\r\n");
    stream.write_all(input.as_bytes()).expect("write failed");

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("read from stream failed");
    println!("read from server: {}", str::from_utf8(&buffer).unwrap());
    Ok(())
}
