// src/models.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "responseDesc")]
    pub response_desc: String,
    #[serde(rename = "responseData", skip_serializing_if = "Option::is_none")]
    pub response_data: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub field: Option<String>,
    pub filter: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    #[serde(rename = "totalPage")]
    pub total_page: i64,
    pub items: Vec<Item>
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
