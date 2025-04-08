use crate::internal::domain::entities::users::users::{CreateUserRequest, UpdateUserRequest, ListUser, UsersQuery};
use crate::internal::domain::entities::response::Response;
use crate::internal::application::repositories::users::users::{self, DeleteItemError};
use crate::internal::pkg::utils::pagination::PaginationRequest;
use crate::internal::constant::status::{SUCCESS, FAILED_INTERNAL, FAILED_NOT_FOUND, FAILED_EXIST, FAILED_REQUIRED};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::{Error, postgres::PgPool};
use serde_json::json;
use std::collections::HashMap;
use url::form_urlencoded;
use tokio::task;
use bcrypt::{hash, DEFAULT_COST};

pub async fn create_user(
    pool: web::Data<PgPool>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    if req.username.trim().len() <= 0 {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                response_desc: "Username required".to_string(),
                response_data: None,
            }
        );
    }

    if req.email.trim().len() <= 0 {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                response_desc: "Email required".to_string(),
                response_data: None,
            }
        );
    }

    if req.password.trim().len() <= 0 {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                response_desc: "Password required".to_string(),
                response_data: None,
            }
        );
    }

    if let Ok(_) = users::get_user_username(pool.get_ref(), req.username.as_str()).await {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_EXIST.to_string(),
                response_desc: "Username already exist".to_string(),
                response_data: None,
            }
        );
    }

    if let Ok(_) = users::get_user_email(pool.get_ref(), req.email.as_str()).await {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_EXIST.to_string(),
                response_desc: "Email already exist".to_string(),
                response_data: None,
            }
        );
    }

    let password = req.password.clone();
    let hashed = match task::spawn_blocking(move || hash(password, DEFAULT_COST)).await {
        Ok(Ok(hash)) => hash,
        _ => return HttpResponse::InternalServerError()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_INTERNAL.to_string(),
                response_desc: "Internal Server Error".to_string(),
                response_data: None,
            }
        ),
    };

    let mut new_req = req.into_inner();
    new_req.password = hashed;
    match users::create_user(&pool, new_req).await {
        Ok(new_user) => HttpResponse::Ok()
        .json(
            Response {
                response_code: SUCCESS.to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(new_user)),
            }
        ),
        Err(err) => HttpResponse::InternalServerError()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_INTERNAL.to_string(),
                response_desc: err.to_string(),
                response_data: None,
            }
        ),
    }
}

pub async fn get_users(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<UsersQuery>
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
    match users::get_users(pool.get_ref(), pagination, filter_map).await {
        Ok((users, count)) => {
            if count == 0 {
                return HttpResponse::NotFound()
                .json(
                    Response::<serde_json::Value> {
                        response_code: FAILED_NOT_FOUND.to_string(),
                        response_desc: "Not Found".to_string(),
                        response_data: None,
                    }
                );
            }

            let total_page = if count % limit == 0 { count / limit } else { count / limit + 1 };
            let paginated: ListUser = ListUser {
                page,
                limit,
                total: count,
                total_page,
                users,
            };
            HttpResponse::Ok()
            .json(
                Response {
                    response_code: SUCCESS.to_string(),
                    response_desc: "OK".to_string(),
                    response_data: Some(json!(paginated)),
                }
            )
        },
        Err(err) => HttpResponse::InternalServerError()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_INTERNAL.to_string(),
                response_desc: err.to_string(),
                response_data: None,
            }
        ),
    }
}

pub async fn get_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    match users::get_user_detail(pool.get_ref(), id.into_inner()).await {
        Ok(user) => HttpResponse::Ok()
        .json(
            Response {
                response_code: SUCCESS.to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(user)),
            }
        ),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_NOT_FOUND.to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_INTERNAL.to_string(),
                    response_desc: err.to_string(),
                    response_data: None,
                }
            ),
        },
    }
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    if req.username.as_deref().map_or(true, |s| s.trim().is_empty()) {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                response_desc: "Username required".to_string(),
                response_data: None,
            }
        );
    }

    if req.email.as_deref().map_or(true, |s| s.trim().is_empty()) {
        return HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                response_desc: "Email required".to_string(),
                response_data: None,
            }
        );
    }

    let id = id.into_inner();
    if let Some(username) = req.username.as_deref() {
        if let Ok(user) = users::get_user_username(pool.get_ref(), username).await {
            if user.id != id {
                return HttpResponse::BadRequest()
                .json(
                    Response::<serde_json::Value> {
                        response_code: FAILED_EXIST.to_string(),
                        response_desc: "Username already exist".to_string(),
                        response_data: None,
                    }
                );
            }
        }
    }

    if let Some(email) = req.email.as_deref() {
        if let Ok(user) = users::get_user_email(pool.get_ref(), email).await {
            if user.id != id {
                return HttpResponse::BadRequest()
                .json(
                    Response::<serde_json::Value> {
                        response_code: FAILED_EXIST.to_string(),
                        response_desc: "Email already exist".to_string(),
                        response_data: None,
                    }
                );
            }
        }
    }

    let mut new_req = req.into_inner();
    if let Some(pwd) = new_req.password.clone() {
        if !pwd.is_empty() {
            let hashed = match task::spawn_blocking(move || hash(pwd, DEFAULT_COST)).await {
                Ok(Ok(h)) => h,
                _ => return HttpResponse::InternalServerError()
                        .json(
                            Response::<serde_json::Value> {
                                response_code: FAILED_INTERNAL.to_string(),
                                response_desc: "Internal Server Error".to_string(),
                                response_data: None,
                            }
                        ),
            };
            new_req.password = Some(hashed);
        }
    }
    match users::update_user(pool.get_ref(), id, new_req).await {
        Ok(user) => HttpResponse::Ok()
        .json(
            Response {
                response_code: SUCCESS.to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(user)),
            }
        ),
        Err(err) => match err {
            Error::RowNotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_NOT_FOUND.to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_INTERNAL.to_string(),
                    response_desc: err.to_string(),
                    response_data: None,
                }
            ),
        },
    }
}

pub async fn delete_user(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    match users::delete_user(pool.get_ref(), id.into_inner()).await {
        Ok(_) => HttpResponse::Ok()
        .json(
            Response::<serde_json::Value> {
                response_code: SUCCESS.to_string(),
                response_desc: "OK".to_string(),
                response_data: None,
            }
        ),
        Err(err) => match err {
            DeleteItemError::NotFound => HttpResponse::NotFound()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_NOT_FOUND.to_string(),
                    response_desc: "Not Found".to_string(),
                    response_data: None,
                }
            ),
            _ => HttpResponse::InternalServerError()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_INTERNAL.to_string(),
                    response_desc: "Internal Server Error".to_string(),
                    response_data: None,
                }
            ),
        },
    }
}
