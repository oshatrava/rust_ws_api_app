use tracing::info;

mod app;
mod database;
mod logger;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let app: axum::Router = app::create_app().await;

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());    
    let addr = std::net::SocketAddr::new(
        [127, 0, 0, 1].into(), 
        port.parse().expect("Failed to parse port")
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to establish connection");

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
