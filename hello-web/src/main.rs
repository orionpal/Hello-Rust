use axum::{
    Router,
    routing::get,
};

async fn hello_world() -> &'static str {
    "Hello, World!"
}
fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
}

#[tokio::main]
async fn main() {
    // From https://docs.rs/axum/latest/axum/fn.serve.html
    let port_addr = "0.0.0.0:3000"; // TODO: put this in config
    println!("Trying to host Rust Backend at {}", port_addr);
    let router = init_router();
    let listener = tokio::net::TcpListener::bind(port_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
