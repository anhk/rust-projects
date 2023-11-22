use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 33333)]
    port: u16,
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = vec![0; 8192];
    loop {
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {
            return;
        }
        socket
            .write_all(&buf[0..n])
            .await
            .expect("write data to socket failed");
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = format!(":::{}", args.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listen on {}", args.port);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}
