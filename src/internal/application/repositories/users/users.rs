use crate::internal::domain::entities::users::users::{CreateUser, User, UpdateUser};
use crate::internal::pkg::utils::pagination::PaginationRequest;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

#[allow(dead_code)]
pub enum DeleteItemError {
    NotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for DeleteItemError {
    fn from(err: sqlx::Error) -> Self {
        DeleteItemError::DatabaseError(err)
    }
}

pub async fn create_user(pool: &PgPool, new_user: CreateUser) -> Result<User, sqlx::Error> {
    let rec = sqlx::query_as::<_, User>(
"INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, password",
    )
    .bind(new_user.username)
    .bind(new_user.email)
    .bind(new_user.password)
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn get_users(
    pool: &PgPool,
    pagination: PaginationRequest,
    filter: HashMap<String, String>
) -> Result<(Vec<User>, i64), sqlx::Error> {
    let valid_sort = match pagination.field.as_str() {
        "id" | "username" | "email" => pagination.field.clone(),
        _ => "id".to_string(),
    };
    let valid_order = if pagination.sort.to_uppercase() == "DESC" { "DESC" } else { "ASC" };
    let limit = pagination.limit;
    let offset = (pagination.page - 1) * pagination.limit;

    // Build dynamic WHERE clause based on filter parameters
    let mut where_clauses = Vec::new();
    if let Some(id) = filter.get("id") {
        // Assuming the id is stored as integer, we cast it to text for comparison
        where_clauses.push(format!("CAST(id AS TEXT) = '{}'", id));
    }
    if let Some(username) = filter.get("username") {
        where_clauses.push(format!("username ILIKE '%{}%'", username));
    }
    if let Some(email) = filter.get("email") {
        where_clauses.push(format!("email ILIKE '%{}%'", email));
    }
    let where_clause = if where_clauses.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };
    
    let query = format!(
        "SELECT id, username, email, password FROM users {} ORDER BY {} {} LIMIT {} OFFSET {}",
        where_clause, valid_sort, valid_order, limit, offset
    );
    let users = sqlx::query_as::<_, User>(&query)
        .fetch_all(pool)
        .await?;
    let count_query = format!(
        "SELECT COUNT(*) FROM users {}",
        where_clause
    );
    let count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await?;
    Ok((users, count))
}

pub async fn get_user(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, username, email, password FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_name(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, username, email, password FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, username, email, password FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update_user(
    pool: &PgPool,
    id: i32,
    update: UpdateUser,
) -> Result<User, sqlx::Error> {
    // Retrieve current item first.
    let current = get_user(pool, id).await?;
    let new_username = update.username.unwrap_or(current.username);
    let new_email = update.email.unwrap_or(current.email);
    let new_password = update.password.or(current.password);

    let user = sqlx::query_as::<_, User>(
"UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, username, email, password"
    )
    .bind(new_username)
    .bind(new_email)
    .bind(new_password)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), DeleteItemError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(DeleteItemError::NotFound);
    }

    Ok(())
}
