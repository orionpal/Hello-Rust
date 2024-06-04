mod routes;
mod database;

use routes::{
    default::hello_world,
    user::hello_user,
    greet::greet,
};
use axum::{
    Router,
    routing::get,
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // Launch Backend
    // From https://docs.rs/axum/latest/axum/fn.serve.html
    let port_addr = "0.0.0.0:8080"; // TODO: put this in config
    println!("Trying to host Rust Backend at {}", port_addr);
    let router = init_router();
    let listener = tokio::net::TcpListener::bind(port_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}



// Define Paths here
fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/hello/:name", get(hello_user))
        .route("/greet", axum::routing::post(greet))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}