use axum::{
    extract::Path,
    response::Json,
};
use serde_json::json;
pub async fn hello_user(Path(name): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "message": format!("Hello, {}!", name) }))
}