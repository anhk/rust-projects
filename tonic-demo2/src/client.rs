use std::thread;
use tokio::sync::mpsc;

use helloworld::{greeter_client::GreeterClient, Message};
use tokio_stream::wrappers::ReceiverStream;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
    let (tx, rx) = mpsc::channel(4);
    let stream = ReceiverStream::new(rx);
    let request = tonic::Request::new(stream);

    let x = client.say_hello(request).await;
    let mut response = x.into_iter();
    // let mut response = client.say_hello(request).await?.into_inner();

    _ = tx
        .send(Message {
            name: "sss".to_string(),
        })
        .await;
    // .await?;

    _ = tx
        .send(Message {
            name: "sss".to_string(),
        })
        .await;

    while let Some(msg) = response.next() {
        println!("=====");
        let mut stream = msg.into_inner();

        while let Some(msg) = stream.message().await.unwrap() {
            println!("{:?}", msg.name);
        }
        // while let Some(msg) = stream.message().await {
        //     println!("{:?}", msg);
        // }
        // println!("{:?}", stream.message());
    }
    println!("pause");
    thread::park();

    // let out_stream = ReceiverStream::new(rx);

    // let response = client.say_hello(out_stream).await;

    // for _i in 0..5 {
    //     _ = tx
    //         .send(Message {
    //             name: String::from("msg"),
    //         })
    //         .await;
    // }

    // let mut in_stream = response.into_iter();
    // while let Some(r) = in_stream.next().await {}

    // for idx in 0..5 {
    //     let msg = in_stream.next();

    //     match msg {
    //         Some(v) => match v {
    //             _ => {
    //                 let x = v.into();
    //                 println!("{:?}", x)
    //             }
    //         },
    //         None => {
    //             println!("break at {}", idx);
    //             break;
    //         }
    //     }
    // }
    println!("exit")
}
