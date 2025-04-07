use crate::internal::domain::entities::auth::login::{LoginRequest, Claims, Token};
use crate::internal::domain::entities::response::Response;
use crate::internal::constant::status::{SUCCESS, FAILED_AUTHORIZED, FAILED_INTERNAL, FAILED_REQUIRED};
use actix_web::{HttpResponse, Responder, web, Error};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPool;
use serde_json::json;
use crate::config::settings::CONFIG;

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    if req.username == "admin" && req.password == "password" {
        let claims = Claims {
            sub: req.username.clone(),
            name: "test".to_string(),
            exp: 10000000000, // Expiration timestamp
        };

        let token: String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(CONFIG.secret_key_jwt.clone().as_ref())) {
            Ok(token) => token,
            Err(err) => return Ok::<HttpResponse, Error>(HttpResponse::InternalServerError().json(
                Response::<serde_json::Value> {
                    response_code: FAILED_INTERNAL.to_string(),
                    response_desc: err.to_string(),
                    response_data: None,
                }
            )),
        };

        let data = Token {
            token
        };

        Ok(HttpResponse::Ok()
        .json(
            Response {
                response_code: SUCCESS.to_string(),
                response_desc: "OK".to_string(),
                response_data: Some(json!(data)),
            }
        ))
        } else {
            if req.username.trim().len()  <= 0 || req.password.trim().len() <= 0 {
                return Ok::<HttpResponse, actix_web::Error>(HttpResponse::Unauthorized()
                .json(
                    Response::<serde_json::Value> {
                        response_code: FAILED_REQUIRED.to_string(),
                        response_desc: "username or password required".to_string(),
                        response_data: None,
                    }
                ));
            }

            Ok::<HttpResponse, actix_web::Error>(HttpResponse::Unauthorized()
            .json(
                Response::<serde_json::Value> {
                    response_code: FAILED_AUTHORIZED.to_string(),
                    response_desc: "Unauthorized".to_string(),
                    response_data: None,
                }
            ))
        }
}
