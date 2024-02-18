use std::net::SocketAddr;
use tracing::info;
use dotenv::dotenv;

mod app;
mod database;
mod logger;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app: axum::Router = app::create_app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to start server");

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
