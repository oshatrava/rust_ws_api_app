use tracing::info;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init(database_url: &str) -> Pool<Postgres> {
    let pg_poll = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .unwrap_or_else(|err| panic!("Failed to connect to the database: {}", err));

    info!("Connected to the database");

    pg_poll
}