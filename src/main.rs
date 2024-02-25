use tracing::info;

mod app;
mod app_config;
mod database;
mod logger;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let config = app_config::AppConfig::new();

    let app: axum::Router = app::create_app(&config).await;

    let addr = std::net::SocketAddr::new(
        [127, 0, 0, 1].into(), 
        config.get_port(),
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to establish connection");

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
