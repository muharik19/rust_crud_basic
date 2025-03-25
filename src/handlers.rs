// src/handlers.rs
use crate::models::{CreateItem, UpdateItem, Response};
use crate::repository::{self, DeleteItemError}; // Ensure repository is imported
use actix_web::{HttpResponse, Responder, web};
use sqlx::{Error, postgres::PgPool};
use serde_json::json;

// Handler to create a new item
pub async fn create_item_handler(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match repository::create_item(&pool, item.into_inner()).await {
        Ok(new_item) => HttpResponse::Ok()
        .json(
            Response {
                response_code: "00".to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(new_item)),
            }
        ),
        Err(err) => HttpResponse::InternalServerError()
        .json(
            Response::<serde_json::Value> {
                response_code: "01".to_string(),
                response_desc: err.to_string(),
                response_data: None,
            }
        ),
    }
}

// Handler to get all items
pub async fn get_items_handler(pool: web::Data<PgPool>) -> impl Responder {
    match repository::get_items(&pool).await {
        Ok(items) => HttpResponse::Ok()
        .json(
            Response {
                response_code: "00".to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(items)),
            }
        ),
        Err(err) => HttpResponse::InternalServerError()
        .json(
            Response::<serde_json::Value> {
                response_code: "01".to_string(),
                response_desc: err.to_string(),
                response_data: None,
            }
        ),
    }
}

// Handler to get a single item by id
pub async fn get_item_handler(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> impl Responder {
    match repository::get_item(&pool, item_id.into_inner()).await {
        Ok(item) => HttpResponse::Ok()
        .json(
            Response {
                response_code: "00".to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(item)),
            }
        ),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: "02".to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: "01".to_string(),
                    response_desc: err.to_string(),
                    response_data: None,
                }
            ),
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
        Ok(item) => HttpResponse::Ok()
        .json(
            Response {
                response_code: "00".to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(item)),
            }
        ),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: "02".to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: "01".to_string(),
                    response_desc: err.to_string(),
                    response_data: None,
                }
            ),
        },
    }
}

// Handler to delete an item by id
pub async fn delete_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
) -> impl Responder {
    match repository::delete_item(&pool, item_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok()
        .json(
            Response::<serde_json::Value> {
                response_code: "00".to_string(),
                response_desc: "OK".to_string(),
                response_data: None,
            }
        ),
        Err(err) => match err {
            DeleteItemError::NotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: "02".to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: "01".to_string(),
                    response_desc: "Internal Server Error".to_string(),
                    response_data: None,
                }
            ),
        },
    }
}
