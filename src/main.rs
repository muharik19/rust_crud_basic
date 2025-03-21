// src/main.rs
mod config;
mod handlers;
mod models;
mod repository;
mod routes;

use actix_web::{App, HttpServer};
use config::CONFIG;
use sqlx::postgres::PgPoolOptions; // our config singleton

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and initialize logger
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = CONFIG.database_url.clone();

    // Initialize Postgres connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to database");

    // Create table if it does not exist.
    // In a full project you’d likely use migrations.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Database connection pooling successfully");

    // Wrap pool in actix_web::Data so it can be shared among handlers.
    let pool_data = actix_web::web::Data::new(pool);

    println!("Serving Rest Http on 0.0.0.0: 9009");

    // Start HTTP server on port 9009.
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 9009))?
    .run()
    .await
}
