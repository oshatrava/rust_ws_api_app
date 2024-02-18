use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{routing::get, Router};
use serde_json::json;
use std::sync::Arc;

use crate::app::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().route("/rooms", get(get_rooms))
}

async fn get_rooms() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rooms = vec!["1", "2", "3"];

    let response = json!({
        "status": "success",
        "data": rooms,
    });

    Ok(Json(response))
}
