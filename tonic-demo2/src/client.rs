use helloworld::greeter_client::GreeterClient;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    let mut _client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
}
