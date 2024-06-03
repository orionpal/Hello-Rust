use serde::{
    Deserialize,
    Serialize
};
use serde_json::json;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
#[derive(Deserialize)]
pub struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
pub struct GreetResponse {
    message: String,
}

pub async fn greet(Json(payload): Json<GreetRequest>) -> impl IntoResponse {
    if payload.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Name cannot be empty" })),
        );
    }

    let response = GreetResponse {
        message: format!("Hello, {}!", payload.name),
    };
    (StatusCode::OK, Json(json!(response)))
}