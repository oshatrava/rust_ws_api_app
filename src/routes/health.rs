use axum::{Router, response::IntoResponse, Json, routing::get};
use serde_json::json;

pub fn create_router() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    let resp = json!({
        "status": "success",
        "message": "OK",
    });

    Json(resp)
}