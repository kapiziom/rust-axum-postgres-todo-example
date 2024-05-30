use chrono::{DateTime, Utc};
use serde::{Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct TodoListModel {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
    #[schema(value_type = Option<String>)]
    pub date_created_utc: DateTime<Utc>
}