use crate::internal::constant::status::FAILED_AUTHORIZED;
use crate::internal::domain::entities::response::Response;
use crate::internal::domain::entities::auth::login::Claims;
use crate::config::settings::CONFIG;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
use std::rc::Rc;
use std::time::Duration;
use std::task::{Context, Poll};

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    fn poll_ready(&self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            // Check for a required header
            if !req.headers().contains_key("Authorization") {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized().json(
                        Response::<serde_json::Value> {
                            response_code: FAILED_AUTHORIZED.to_string(),
                            response_desc: "Authorization is missing.".to_string(),
                            response_data: None,
                        }
                    )
                ).map_into_boxed_body());
            }

            if let Some(auth_header) = req.headers().get("Authorization") {
                match auth_header.to_str() {
                    Ok(header_value) => {
                        let token = header_value.strip_prefix("Bearer ").unwrap_or("").to_string();
                        if token.is_empty() {
                            return Ok(req.into_response(
                                HttpResponse::Unauthorized().json(
                                    Response::<serde_json::Value> {
                                        response_code: FAILED_AUTHORIZED.to_string(),
                                        response_desc: "Invalid Bearer token.".to_string(),
                                        response_data: None,
                                    }
                                )
                            ).map_into_boxed_body());
                        }

                        let decoding_key = DecodingKey::from_secret(CONFIG.secret_key_jwt.clone().as_ref());
                        let validation = Validation::new(Algorithm::HS256);

                        let token_data = match decode::<Claims>(&token, &decoding_key, &validation) {
                            Ok(data) => data,
                            Err(err) => {
                                if let ErrorKind::ExpiredSignature = err.kind() {
                                    return Ok(req.into_response(
                                        HttpResponse::Unauthorized().json(
                                            Response::<serde_json::Value> {
                                                response_code: FAILED_AUTHORIZED.to_string(),
                                                response_desc: "Token has expired.".to_string(),
                                                response_data: None,
                                            }
                                        )
                                    ).map_into_boxed_body());
                                } else if let ErrorKind::InvalidToken = err.kind() {
                                    return Ok(req.into_response(
                                        HttpResponse::Unauthorized().json(
                                            Response::<serde_json::Value> {
                                                response_code: FAILED_AUTHORIZED.to_string(),
                                                response_desc: "Token is invalid.".to_string(),
                                                response_data: None,
                                            }
                                        )
                                    ).map_into_boxed_body());
                                } else {
                                    return Ok(req.into_response(
                                        HttpResponse::Unauthorized().json(
                                            Response::<serde_json::Value> {
                                                response_code: FAILED_AUTHORIZED.to_string(),
                                                response_desc: format!("Token verification failed: {}.", err),
                                                response_data: None,
                                            }
                                        )
                                    ).map_into_boxed_body());
                                }
                            }
                        };

                        let ctx = Claims {
                            sub: token_data.claims.sub,
                            name: token_data.claims.name,
                            exp: token_data.claims.exp,
                        };
                        req.extensions_mut().insert(ctx);
                    }
                    Err(_) => {
                        return Ok(req.into_response(
                            HttpResponse::Unauthorized().json(
                                Response::<serde_json::Value> {
                                    response_code: FAILED_AUTHORIZED.to_string(),
                                    response_desc: "Invalid Authorization header.".to_string(),
                                    response_data: None,
                                }
                            )
                        ).map_into_boxed_body());
                    }
                }
            }
            service.call(req).await.map(ServiceResponse::map_into_boxed_body)
        })
    }
}

// pub async fn is_authorized(token: &str) -> Result<bool, String> {
//     let claims = jwt_decode(token, secret).await?;
//     // Optional: check if claims are acceptable
//     if claims.sub.is_empty() {
//         return Ok(false);
//     }
//     Ok(true)
// }

pub fn parse_jwt_exp(exp: &str) -> Option<Duration> {
    let (num_part, unit_part) = exp.trim().split_at(exp.len() - 1);
    let number: u64 = num_part.parse().ok()?;
    match unit_part {
        "s" => Some(Duration::from_secs(number)),          // seconds
        "m" => Some(Duration::from_secs(number * 60)),     // minutes
        "h" => Some(Duration::from_secs(number * 3600)),   // hours
        "d" => Some(Duration::from_secs(number * 86400)),  // days
        _ => None, // invalid format
    }
}
