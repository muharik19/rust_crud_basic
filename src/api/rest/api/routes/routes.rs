use crate::internal::application::controllers::items::items;
use crate::internal::application::controllers::auth::login;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("", web::post().to(items::create_item_controller))
            .route("", web::get().to(items::get_items_controller))
            .route("/{id}", web::get().to(items::get_item_controller))
            .route("/{id}", web::put().to(items::update_item_controller))
            .route("/{id}", web::delete().to(items::delete_item_controller)),
    );

    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login::login_controller)),
    );
}
