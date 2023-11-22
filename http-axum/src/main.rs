use axum::{
    routing::{get, post},
    Router,
};

async fn root() -> String {
    String::from("Hello World!")
}

async fn version() -> String {
    String::from("v1.0.0")
}

async fn update_version() -> String {
    String::from("v1.0.0")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/version", get(version).post(update_version))
        .route("/mod", post(|| async {}));

    let addr = "127.0.0.1:30000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
