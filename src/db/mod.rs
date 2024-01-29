pub mod note_schema;
pub mod note_model;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn setup_database() -> Pool<Postgres> {
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