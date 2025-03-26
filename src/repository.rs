use crate::models::{CreateItem, Item, UpdateItem};
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

// Create a new item
pub async fn create_item(pool: &PgPool, new_item: CreateItem) -> Result<Item, sqlx::Error> {
    let rec = sqlx::query_as::<_, Item>(
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id, name, description",
    )
    .bind(new_item.name)
    .bind(new_item.description)
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn get_items(
    pool: &PgPool,
    pagination: PaginationRequest,
    filter: HashMap<String, String>
) -> Result<(Vec<Item>, i64), sqlx::Error> {
    let valid_sort = match pagination.field.as_str() {
        "id" | "name" | "description" => pagination.field.clone(),
        _ => "id".to_string(),
    };
    let valid_order = if pagination.sort.to_uppercase() == "DESC" { "DESC" } else { "ASC" };
    let item_limit = pagination.limit;
    let item_offset = (pagination.page - 1) * pagination.limit;

    // Build dynamic WHERE clause based on filter parameters
    let mut where_clauses = Vec::new();
    if let Some(id) = filter.get("id") {
        // Assuming the id is stored as integer, we cast it to text for comparison
        where_clauses.push(format!("CAST(id AS TEXT) = '{}'", id));
    }
    if let Some(name) = filter.get("name") {
        where_clauses.push(format!("name ILIKE '%{}%'", name));
    }
    if let Some(description) = filter.get("description") {
        where_clauses.push(format!("description ILIKE '%{}%'", description));
    }
    let where_clause = if where_clauses.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };
    
    let query = format!(
        "SELECT id, name, description FROM items {} ORDER BY {} {} LIMIT {} OFFSET {}",
        where_clause, valid_sort, valid_order, item_limit, item_offset
    );
    let items = sqlx::query_as::<_, Item>(&query)
        .fetch_all(pool)
        .await?;
    let count_query = format!(
        "SELECT COUNT(*) FROM items {}",
        where_clause
    );
    let count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await?;
    Ok((items, count))
}

// Retrieve a single item by id
pub async fn get_item(pool: &PgPool, item_id: i32) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items WHERE id = $1")
        .bind(item_id)
        .fetch_one(pool)
        .await?;
    Ok(item)
}

// Update an item by id (partial update)
pub async fn update_item(
    pool: &PgPool,
    item_id: i32,
    update: UpdateItem,
) -> Result<Item, sqlx::Error> {
    // Retrieve current item first.
    let current = get_item(pool, item_id).await?;
    let new_name = update.name.unwrap_or(current.name);
    let new_description = update.description.or(current.description);

    let item = sqlx::query_as::<_, Item>(
        "UPDATE items SET name = $1, description = $2 WHERE id = $3 RETURNING id, name, description"
    )
    .bind(new_name)
    .bind(new_description)
    .bind(item_id)
    .fetch_one(pool)
    .await?;
    Ok(item)
}

// Delete an item by id
pub async fn delete_item(pool: &PgPool, item_id: i32) -> Result<(), DeleteItemError> {
    let result = sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(item_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(DeleteItemError::NotFound);
    }

    Ok(())
}
