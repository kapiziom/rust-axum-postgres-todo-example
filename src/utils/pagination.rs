use serde::{Deserialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct Pager {
    pub offset: i32,
    pub page_size: i32,
    pub sort_by: String,
    pub sort_order: String
}