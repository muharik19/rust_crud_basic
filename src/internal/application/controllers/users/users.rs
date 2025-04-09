use crate::internal::domain::entities::users::users::{CreateUserRequest, UpdateUserRequest, UsersQuery};
use crate::internal::application::usecases::users::users::{create_user, get_users, get_user, update_user, delete_user};
use crate::internal::domain::entities::response::Response;
use crate::internal::constant::status::FAILED_AUTHORIZED;
use crate::middlewares::jwt::jwt_decode;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::postgres::PgPool;

pub async fn create_user_controller(
    pool: web::Data<PgPool>,
    req: web::Json<CreateUserRequest>,
    http_request: HttpRequest,
) -> HttpResponse {
    let req_headers = http_request.headers();
    // Check for a required header
    if !req_headers.contains_key("Authorization") {
    return HttpResponse::Unauthorized().json(
            Response::<serde_json::Value> {
                response_code: FAILED_AUTHORIZED.to_string(),
                response_desc: "Authorization is missing.".to_string(),
                response_data: None,
            }
        );
    }

    let auth_header = req_headers.get("Authorization");
    let auth: &str = auth_header.expect("Authorization must be set").to_str().unwrap();

    let claims = match jwt_decode(auth).await {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::Unauthorized()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_AUTHORIZED.to_string(),
                response_desc: format!("{}", e.to_string()),
                response_data: None,
            }
        ),
    };

    // match is_authorized(auth, CONFIG.secret_key_jwt.clone().as_ref()).await {
    //     Ok(true) => println!("✅ Authorized"),
    //     Ok(false) => return HttpResponse::Unauthorized().json(
    //         Response::<serde_json::Value> {
    //             response_code: FAILED_AUTHORIZED.to_string(),
    //             response_desc: "Unauthorized".to_string(),
    //             response_data: None,
    //         }
    //     ),
    //     Err(e) => return HttpResponse::Unauthorized()
    //     .json(
    //         Response::<serde_json::Value> {
    //             response_code: FAILED_AUTHORIZED.to_string(),
    //             response_desc: format!("{}", e.to_string()),
    //             response_data: None,
    //         }
    //     ),
    // }

    create_user(pool, req, claims).await
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
