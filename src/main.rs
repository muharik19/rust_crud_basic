mod config;
mod handlers;
mod models;
mod repository;
mod routes;
mod internal;

use actix_web::{App, HttpServer, web, middleware::DefaultHeaders, http::header};
use crate::internal::pkg::database::sql::postgres::create_pool;
use config::settings::CONFIG;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and initialize logger
    dotenv::dotenv().ok();
    env_logger::init();

    // Initialize Postgres connection pool
    let pool = create_pool().await;

    // Wrap pool in actix_web::Data so it can be shared among handlers.
    let pool_data = web::Data::new(pool);

    let port: u16 = CONFIG.port.parse().expect("Invalid port");
    
    println!("Serving Rest Http on 0.0.0.0: {}", port);

    // Start HTTP server on port 9009.
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(DefaultHeaders::new()
            .add((header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "false"))
            .add((header::ACCESS_CONTROL_ALLOW_HEADERS, "Accept, Content-Type, Content-Length, Accept-Encoding, Authorization, Origin, Cookie, Timestamp"))
            .add((header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS"))
            .add((header::CACHE_CONTROL, "no-store"))
            .add((header::CONTENT_SECURITY_POLICY, "default-src 'self'"))
            .add((header::CONTENT_TYPE, "application/json"))
            .add((header::STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains"))
            .add((header::VARY, "Origin"))
            .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        )
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
