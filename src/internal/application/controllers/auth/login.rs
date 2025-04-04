use crate::internal::domain::entities::auth::login::LoginRequest;
use crate::internal::application::usecases::auth::login::login;
use actix_web::{Responder, web};
use sqlx::postgres::PgPool;

pub async fn login_controller(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    login(pool, req).await
}