use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}