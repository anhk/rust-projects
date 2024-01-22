pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use std::net::ToSocketAddrs;

use crate::helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::Message;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Debug)]
struct MyGreetServer {}

#[tonic::async_trait]
impl Greeter for MyGreetServer {
    type SayHelloStream = ReceiverStream<Result<Message, Status>>;

    async fn say_hello(
        &self,
        request: Request<Streaming<Message>>,
    ) -> Result<Response<Self::SayHelloStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(msg) = in_stream.next().await {
                match msg {
                    Ok(v) => match tx.send(Ok(v)).await {
                        Ok(_) => (),
                        Err(_e) => break,
                    },
                    Err(e) => match tx.send(Err(e)).await {
                        Ok(_) => (),
                        Err(_e) => break,
                    },
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(out_stream as Self::SayHelloStream))
    }
}

#[tokio::main]
async fn main() {
    let server = MyGreetServer {};
    Server::builder()
        .add_service(GreeterServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    println!("Hello, world!");
}
