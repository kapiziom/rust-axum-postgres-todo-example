use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub date_created_utc: Option<DateTime<Utc>>,
    pub date_completed_utc: Option<DateTime<Utc>>,
}