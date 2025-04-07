use crate::internal::domain::entities::users::users::{CreateUser, UpdateUser, UsersQuery};
use crate::internal::application::usecases::users::users::{create_user, get_users, get_user, update_user, delete_user};
use actix_web::{HttpRequest, Responder, web};
use sqlx::postgres::PgPool;

pub async fn create_user_controller(
    pool: web::Data<PgPool>,
    req: web::Json<CreateUser>,
) -> impl Responder {
    create_user(pool, req).await
}

pub async fn get_users_controller(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<UsersQuery>
) -> impl Responder {
    get_users(pool, req, params).await
}

pub async fn get_user_controller(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    get_user(pool, id).await
}

pub async fn update_user_controller(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    update: web::Json<UpdateUser>,
) -> impl Responder {
    update_user(pool, id, update).await
}

pub async fn delete_user_controller(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    delete_user(pool, id).await
}
