mod log;
mod db;
mod route;
mod handlers;

use std::collections::{HashMap, HashSet};
use std::{net::SocketAddr, sync::Mutex};
use std::sync::Arc;

use dotenv::dotenv;
use axum::http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, 
};
use tokio::sync::broadcast;
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use route::create_router;
use db::setup_database;
use log::setup_tracing;

struct AppState {
    db: Pool<Postgres>,
    rooms: Mutex<HashMap<String, RoomState>>,
}

struct RoomState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

impl RoomState {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashSet::new()),
            tx: broadcast::channel(69).0,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    setup_tracing();
    let pool = setup_database().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = Arc::new(AppState { 
        db: pool.clone(),
        rooms: Mutex::new(HashMap::new()), 
    });
    let app = create_router(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    tracing::info!("🚀 Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
