use actix_web::{App, HttpServer, web, middleware::DefaultHeaders, http::header};
use crate::config::settings::CONFIG;
use crate::api::rest::api::routes::routes::init_routes;

pub async fn start_server(pool_data: web::Data<sqlx::Pool<sqlx::Postgres>>) -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Invalid port");

    println!("Serving Rest Http on 0.0.0.0:{}", port);
    
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
            .configure(init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}