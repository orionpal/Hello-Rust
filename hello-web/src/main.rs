mod routes;
use eframe::egui;
use routes::{
    default::hello_world,
    user::hello_user,
    greet::greet,
};
use axum::{
    Router,
    routing::get,
};


// Define Paths here
fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/hello/:name", get(hello_user))
        .route("/greet", axum::routing::post(greet))
}

#[tokio::main]
async fn main() {
    // Launch Backend
    // From https://docs.rs/axum/latest/axum/fn.serve.html
    let port_addr = "0.0.0.0:3000"; // TODO: put this in config
    println!("Trying to host Rust Backend at {}", port_addr);
    let router = init_router();
    let listener = tokio::net::TcpListener::bind(port_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    // Launch Frontend
    // Launch the web app
    eframe::start_web(
        "the_canvas_id", // id of the html canvas element
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
