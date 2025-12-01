use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(root));

}

async fn root() -> &'static str {
    "Hello, world!"
}