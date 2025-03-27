use crate::internal::domain::entities::items::items::{CreateItem, UpdateItem, Items, ItemsQuery};
use crate::internal::domain::entities::response::Response;
use crate::internal::application::repositories::items::items::{self, DeleteItemError};
use crate::internal::pkg::utils::pagination::PaginationRequest;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::{Error, postgres::PgPool};
use serde_json::json;
use std::collections::HashMap;
use url::form_urlencoded;

pub async fn create_item(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> impl Responder {
    match items::create_item(&pool, item.into_inner()).await {
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

pub async fn get_items(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<ItemsQuery>
) -> impl Responder {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let sort = params.sort.as_deref().unwrap_or("name");
    let field = params.field.as_deref().unwrap_or("ASC");
    let pagination = PaginationRequest::new(limit, page, field, sort);
    let mut filter_map = params.filter.clone().unwrap_or_default();
    let query_str = req.query_string();
    let query: HashMap<String, String> = form_urlencoded::parse(query_str.as_bytes()).into_owned().collect();
    for (key, value) in query {
        if key.starts_with("filter[") && key.ends_with("]") {
            let inner_key = &key[7..key.len()-1];
            filter_map.insert(inner_key.to_string(), value.clone());
            // println!("Received filter {}: {}", inner_key, value);
        }
    }
    // if filter_map.is_empty() {
    //     println!("Filter is not provided");
    // }
    match items::get_items(pool.get_ref(), pagination, filter_map).await {
        Ok((items, count)) => {
            let total_page = if count % limit == 0 { count / limit } else { count / limit + 1 };
            let paginated: Items = Items {
                page,
                limit,
                total: count,
                total_page,
                items,
            };
            HttpResponse::Ok()
            .json(
                Response {
                    response_code: "00".to_string(),
                    response_desc: "OK".to_string(),
                    response_data: Some(json!(paginated)),
                }
            )
        },
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

pub async fn get_item(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>
) -> impl Responder {
    match items::get_item(pool.get_ref(), item_id.into_inner()).await {
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

pub async fn update_item(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
    update: web::Json<UpdateItem>,
) -> impl Responder {
    match items::update_item(pool.get_ref(), item_id.into_inner(), update.into_inner()).await {
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

pub async fn delete_item(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
) -> impl Responder {
    match items::delete_item(pool.get_ref(), item_id.into_inner()).await {
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
