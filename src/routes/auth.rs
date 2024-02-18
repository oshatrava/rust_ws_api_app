use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Router};
use tracing::debug;

use crate::app::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register() -> impl IntoResponse {
    debug!("Registering user");
    unimplemented!()
}

async fn login() -> impl IntoResponse {
    debug!("Logging in user");

    unimplemented!()
}
