mod handler;
mod model;
mod schema;
mod route;

use std::sync::Arc;

use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, 
};
use tower_http::cors::CorsLayer;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt as _};

use route::create_router;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pg_connection = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;
    
    let pool = match pg_connection {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = create_router(app_state).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
