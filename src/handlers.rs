// src/handlers.rs
use crate::models::{CreateItem, UpdateItem};
use crate::repository::{self, DeleteItemError}; // Ensure repository is imported
use actix_web::{HttpResponse, Responder, web};
use sqlx::{Error, postgres::PgPool};

// Handler to create a new item
pub async fn create_item_handler(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match repository::create_item(&pool, item.into_inner()).await {
        Ok(new_item) => HttpResponse::Ok().json(new_item),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "responseCode": "01",
            "responseDesc": err.to_string()
        })),
    }
}

// Handler to get all items
pub async fn get_items_handler(pool: web::Data<PgPool>) -> impl Responder {
    match repository::get_items(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "responseCode": "01",
            "responseDesc": err.to_string()
        })),
    }
}

// Handler to get a single item by id
pub async fn get_item_handler(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> impl Responder {
    match repository::get_item(&pool, item_id.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound().json(serde_json::json!({
                "responseCode": "02",
                "responseDesc": "Not Found"
            })),
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "responseCode": "01",
                "responseDesc": err.to_string()
            })),
        },
    }
}

// Handler to update an item by id
pub async fn update_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
    update: web::Json<UpdateItem>,
) -> impl Responder {
    match repository::update_item(&pool, item_id.into_inner(), update.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound().json(serde_json::json!({
                "responseCode": "02",
                "responseDesc": "Not Found"
            })),
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "responseCode": "01",
                "responseDesc": err.to_string()
            })),
        },
    }
}

// Handler to delete an item by id
pub async fn delete_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
) -> impl Responder {
    match repository::delete_item(&pool, item_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "responseCode": "00",
            "responseDesc": "OK"
        })),
        Err(err) => match err {
            DeleteItemError::NotFound => HttpResponse::NotFound().json(serde_json::json!({
                "responseCode": "02",
                "responseDesc": "Not Found"
            })),
            _ => HttpResponse::InternalServerError().json(serde_json::json!({
                "responseCode": "01",
                "responseDesc": "Internal server error"
            })),
        },
    }
}
