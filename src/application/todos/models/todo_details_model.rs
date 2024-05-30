use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoDetailsModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    #[schema(value_type = Option<String>)]
    pub date_created_utc: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>)]
    pub date_modified_utc: Option<DateTime<Utc>>,
}