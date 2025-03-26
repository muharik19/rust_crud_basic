use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::config::settings::CONFIG;

pub async fn create_pool() -> Pool<Postgres> {
    let database_url = CONFIG.database_url.clone();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to database");

    println!("Database connection pooling successfully");

    pool
}
