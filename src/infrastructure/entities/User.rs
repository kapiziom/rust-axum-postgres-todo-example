use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub date_created_utc: Option<DateTime<Utc>>,
}