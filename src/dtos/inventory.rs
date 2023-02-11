use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InventoryGetDto {
    pub code: String,
    pub amount: u64,
    pub created_at: Option<DateTime<Utc>>
}

impl InventoryGetDto {
    pub fn new(code: String, amount: u64, created_at: Option<DateTime<Utc>>) -> Self { Self { code, amount, created_at } }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct InventoryPostDto {
    pub code: String,
    pub amount: u64
}