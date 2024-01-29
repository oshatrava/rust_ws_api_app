mod handlers;
mod model;
mod schema;
mod route;

use std::collections::{HashMap, HashSet};
use std::{net::SocketAddr, sync::Mutex};
use std::sync::Arc;

use dotenv::dotenv;
use axum::http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, 
};
use tokio::sync::broadcast;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use route::create_router;

pub struct AppState {
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

fn setup_tracing() {
    let mut layers = Vec::new();

    if true {
        let stdout_log = tracing_subscriber::fmt::layer()
            .pretty()
            .with_filter(filter::LevelFilter::DEBUG)
            .boxed();
        layers.push(stdout_log);
    }

    tracing_subscriber::registry()
        .with(layers)
        .init();
}

async fn setup_database() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pg_connection = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;
    
    match pg_connection {
        Ok(pool) => {
            tracing::info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
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
