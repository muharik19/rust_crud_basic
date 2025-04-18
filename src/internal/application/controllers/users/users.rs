use crate::internal::domain::entities::users::users::{CreateUserRequest, UpdateUserRequest, UsersQuery};
use crate::internal::application::usecases::users::users::{create_user, get_users, get_user, update_user, delete_user};
use actix_web::{HttpRequest, Responder, web};
use sqlx::postgres::PgPool;

pub async fn create_user_controller(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    payload: web::Json<CreateUserRequest>,
) -> impl Responder {
    create_user(pool, http_req, payload).await
}

pub async fn get_users_controller(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
    params: web::Query<UsersQuery>
) -> impl Responder {
    get_users(pool, http_req, params).await
}

pub async fn get_user_controller(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    get_user(pool, id).await
}

pub async fn update_user_controller(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    update: web::Json<UpdateUserRequest>,
) -> impl Responder {
    update_user(pool, id, update).await
}

pub async fn delete_user_controller(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    delete_user(pool, id).await
}
