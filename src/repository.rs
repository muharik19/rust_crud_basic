// src/repository.rs
use crate::models::{CreateItem, Item, UpdateItem};
use sqlx::postgres::PgPool;

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

// Retrieve all items
pub async fn get_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items")
        .fetch_all(pool)
        .await?;
    Ok(items)
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
