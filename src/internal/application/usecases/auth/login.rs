use crate::internal::domain::entities::auth::login::{LoginRequest, Claims, Token};
use crate::internal::domain::entities::response::Response;
use crate::internal::constant::status::{SUCCESS, FAILED_AUTHORIZED, FAILED_INTERNAL, FAILED_REQUIRED};
use crate::internal::application::repositories::users::users::get_user_or;
use crate::config::settings::CONFIG;
use crate::internal::pkg::utils::jwt_exp::parse_jwt_exp;
use actix_web::{HttpResponse, Responder, web, Error};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPool;
use serde_json::json;
use tokio::task;
use bcrypt::verify;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    if req.username.trim().len() > 0 || req.password.trim().len() > 0 {
        if let Ok(user) = get_user_or(pool.get_ref(), req.username.as_str(), req.username.as_str()).await {
            let verified = match task::spawn_blocking(move || verify(req.password.as_str(), &user.password)).await {
                Ok(Ok(true)) => true,
                _ => false,
            };

            if verified {
                let jwt_exp = CONFIG.jwt_exp.clone();
                if let Some(exp_duration) = parse_jwt_exp(&jwt_exp) {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    let exp_timestamp = now + exp_duration;
                    // println!("JWT expires at UNIX timestamp: {}", exp_timestamp.as_secs());
                    let claims = Claims {
                        sub: user.username,
                        name: "test".to_string(),
                        exp: exp_timestamp.as_secs() as usize, // Expiration timestamp
                    };

                    let token: String = match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(CONFIG.secret_key_jwt.clone().as_ref())
                    ) {
                        Ok(token) => token,
                        Err(err) => {
                            return Ok::<HttpResponse, Error>(HttpResponse::InternalServerError().json(
                                Response::<serde_json::Value> {
                                    response_code: FAILED_INTERNAL.to_string(),
                                    response_desc: err.to_string(),
                                    response_data: None,
                                }
                            ));
                        }
                    };

                    let data: Token = Token { token };

                    Ok(HttpResponse::Ok().json(
                        Response {
                            response_code: SUCCESS.to_string(),
                            response_desc: "OK".to_string(),
                            response_data: Some(json!(data)),
                        }
                    ))
                } else {
                    let error_message = format!("Invalid JWT_EXP format: {}", jwt_exp);
                    Ok::<HttpResponse, actix_web::Error>(HttpResponse::InternalServerError().json(
                        Response::<serde_json::Value> {
                            response_code: FAILED_INTERNAL.to_string(),
                            response_desc: error_message,
                            response_data: None,
                        }
                    ))
                }
            } else {
                Ok::<HttpResponse, actix_web::Error>(HttpResponse::Unauthorized().json(
                    Response::<serde_json::Value> {
                        response_code: FAILED_AUTHORIZED.to_string(),
                        response_desc: "Unauthorized".to_string(),
                        response_data: None,
                    }
                ))
            }
        } else {
            Ok::<HttpResponse, actix_web::Error>(HttpResponse::Unauthorized().json(
                Response::<serde_json::Value> {
                    response_code: FAILED_AUTHORIZED.to_string(),
                    response_desc: "Unauthorized".to_string(),
                    response_data: None,
                }
            ))
        }
    } else {
        Ok::<HttpResponse, actix_web::Error>(HttpResponse::BadRequest()
        .json(
            Response::<serde_json::Value> {
                response_code: FAILED_REQUIRED.to_string(),
                    response_desc: "username or password required".to_string(),
                    response_data: None,
            }
        ))
    }
}
