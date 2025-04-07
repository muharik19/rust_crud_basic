use crate::internal::application::controllers::items::items;
use crate::internal::application::controllers::users::users;
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
        web::scope("/users")
            .route("", web::post().to(users::create_user_controller))
            .route("", web::get().to(users::get_users_controller))
            .route("/{id}", web::get().to(users::get_user_controller))
            .route("/{id}", web::put().to(users::update_user_controller))
            .route("/{id}", web::delete().to(users::delete_user_controller)),
    );

    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login::login_controller)),
    );
}
