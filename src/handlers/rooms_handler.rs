use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::AppState;

pub async fn list_rooms_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let rooms = state.rooms.lock().unwrap();
    let rooms = rooms.keys().collect::<Vec<&String>>();

    let response = json!({
        "status": "success",
        "data": rooms,
    });

    Ok(Json(response))
}
