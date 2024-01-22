pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use crate::helloworld::greeter_server::Greeter;
use helloworld::Message;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
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
                    Ok(v) => tx.send(Ok(v)).await.expect(""),
                    Err(e) => tx.send(Err(e)).await.expect(""),
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(out_stream as Self::SayHelloStream))
    }
}

fn main() {
    println!("Hello, world!");
}
