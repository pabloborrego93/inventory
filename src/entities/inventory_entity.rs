use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds_option;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryEntity {
    pub code: String,
    pub amount: u64,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}