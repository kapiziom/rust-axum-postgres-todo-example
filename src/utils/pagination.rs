use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pager {
    pub offset: i32,
    pub page_size: i32,
    pub sort_by: String,
    pub sort_order: String
}