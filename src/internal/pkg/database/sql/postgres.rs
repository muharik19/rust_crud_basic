use crate::config::settings::CONFIG;
use crate::middlewares::logger::init_logger;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use slog::info;

pub async fn create_pool() -> Pool<Postgres> {
    let database_url = CONFIG.database_url.clone();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to database");

    let (_, logger_terminal) = init_logger();
    info!(logger_terminal, "Database connection pooling successfully");

    pool
}
