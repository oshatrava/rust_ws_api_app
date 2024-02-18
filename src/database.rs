use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn setup() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to the database: {}", e.to_string()));

    return pool;
}