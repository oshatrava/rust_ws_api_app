use std::sync::Arc;
use dotenv::dotenv;
use axum::http::header;
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace;

use crate::logger;
use crate::database;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn create_app() -> Router {
    dotenv().ok();

    logger::setup();

    let db_pool = database::setup().await;

    let app_state = AppState {
        db: db_pool,
    };

    Router::new()
        .merge(routes::health::create_router())
        .merge(Router::new().nest(
            "/auth", 
            Router::new()
                .merge(routes::auth::create_router())
                .with_state(Arc::new(app_state.clone())),
        ))
        .merge(Router::new().nest(
            "/api/v1",
            Router::new()
                .merge(routes::notes::create_router())
                .merge(routes::rooms::create_router())
                .with_state(Arc::new(app_state.clone())),
        ))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        .layer(CorsLayer::permissive())
}
