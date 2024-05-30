use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserModel {
    pub email: String,
    #[schema(value_type = Option<String>)]
    pub date_created_utc: Option<DateTime<Utc>>,
}