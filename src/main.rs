mod config;
mod internal;
mod api;
mod middlewares;

use crate::internal::pkg::database::sql::postgres::create_pool;
use crate::api::rest::api::server::start_server;
use actix_web::web::Data;
use dotenv::dotenv;
use env_logger::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and initialize logger
    dotenv().ok();
    init();

    // Initialize Postgres connection pool
    let pool = create_pool().await;

    // Wrap pool in actix_web::Data so it can be shared among handlers.
    let pool_data = Data::new(pool);

    // Start server
    start_server(pool_data).await
}
