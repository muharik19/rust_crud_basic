use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UsersQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub field: Option<String>,
    pub filter: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub users: Vec<User>
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}