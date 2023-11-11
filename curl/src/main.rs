use std::{
    io::{self, BufReader, Read, Write},
    net::TcpStream,
    str,
};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("192.168.64.52:32351")?;

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
