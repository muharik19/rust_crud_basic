use crate::internal::domain::entities::items::items::{CreateItem, UpdateItem, ItemsQuery};
use crate::internal::application::usecases::items::items::{create_item, get_items, get_item, update_item, delete_item};
use actix_web::{HttpRequest, Responder, web};
use sqlx::postgres::PgPool;

pub async fn create_item_controller(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    create_item(pool, item).await
}

pub async fn get_items_controller(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<ItemsQuery>
) -> impl Responder {
    get_items(pool, req, params).await
}

pub async fn get_item_controller(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> impl Responder {
    get_item(pool, item_id).await
}

pub async fn update_item_controller(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
    update: web::Json<UpdateItem>,
) -> impl Responder {
    update_item(pool, item_id, update).await
}

pub async fn delete_item_controller(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
) -> impl Responder {
    delete_item(pool, item_id).await
}
