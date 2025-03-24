// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseDesc")]
    pub response_desc: String,
    #[serde(rename = "responseData", skip_serializing_if = "Option::is_none")]
    pub response_data: Option<T>,
}

// Represents an item stored in the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// Model for creating a new item.
#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}

// Model for updating an existing item.
#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub description: Option<String>,
}
