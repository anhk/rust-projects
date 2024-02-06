use clap::Parser;
use tokio::net::{TcpListener, TcpStream};
use tokio::{io, select};

#[derive(Parser, Debug)]
struct Argument {
    #[arg(short, long, default_value_t = String::from("[::]:1212"))]
    listen: String,

    #[arg(short, long ,default_value_t = String::from("127.0.0.1:1313"))]
    connect: String,
}

async fn run(args: Argument) -> io::Result<()> {
    println!("{:?}", args);
    let listener = TcpListener::bind(args.listen).await?;

    let (client, _) = listener.accept().await?;
    let server = TcpStream::connect(args.connect).await?;

    let (mut eread, mut ewrite) = client.into_split();
    let (mut oread, mut owrite) = server.into_split();

    let e2o = tokio::spawn(async move { io::copy(&mut eread, &mut owrite).await });
    let o2e = tokio::spawn(async move { io::copy(&mut oread, &mut ewrite).await });

    select! {
        val = e2o => println!("c2s done with {:?}", val),
        val = o2e => println!("s2c done with {:?}", val),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Argument::parse();

    run(args).await
}
