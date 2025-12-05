use axum::routing::get;
use axum::routing::Router;
use axum::serve;
use axum::Json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde::Serialize;


#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{addr}");

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    serve::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_user() -> Json<User> {
    let user = User {
        name: "Ivan".to_string(),
        age: 20,
    };

    Json(user)
}