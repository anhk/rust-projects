use std::net::UdpSocket;
use std::str;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// target to connect
    target: String,

    #[arg(short, long, default_value_t = String::from("test"))]
    data: String,

    #[arg(long, default_value_t = 4)]
    datalen: usize,
}

fn expand_data(data: String, datalen: usize) -> String {
    let len = datalen / data.len() + 1;
    let mut r = Vec::with_capacity(len);

    for _i in 0..len {
        r.push(data.as_str());
    }

    let joined = r.join("").as_str().to_string();
    joined[0..datalen].to_string()
}

fn main() {
    let args = Arguments::parse();

    let mut host: &str = &args.target;
    let mut port = 33333;

    match args.target.split_once(':') {
        Some((key, value)) => {
            host = key;
            port = value.parse().expect("invalid port");
        }
        None => {}
    }

    println!(
        "send udp to {}:{}, with data: {}, datalen: {}",
        host, port, args.data, args.datalen
    );

    let data = expand_data(args.data, args.datalen);
    let socket = UdpSocket::bind("[::]:0").expect("create udp socket failed");
    socket
        .send_to(data.as_bytes(), format!("{}:{}", host, port))
        .expect("udp send failed");

    let mut buff = [0; 65536];
    let (amt, _src) = socket.recv_from(&mut buff).expect("recv from failed");

    let echo = str::from_utf8(&buff[..amt]).unwrap();
    println!("{}", echo);
}
