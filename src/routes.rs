// src/routes.rs
use crate::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("", web::post().to(handlers::create_item_handler))
            .route("", web::get().to(handlers::get_items_handler))
            .route("/{id}", web::get().to(handlers::get_item_handler))
            .route("/{id}", web::put().to(handlers::update_item_handler))
            .route("/{id}", web::delete().to(handlers::delete_item_handler)),
    );
}
