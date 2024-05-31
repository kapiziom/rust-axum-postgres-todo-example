use serde::Deserialize;
use validator_derive::Validate;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodoModel {
    #[validate(length(min = 3, max = 250, message = "Title must be between 3 and 250 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 30, max = 1000, message = "Description must be between 30 and 1000 characters"))]
    pub description: Option<String>,

    pub is_completed: Option<bool>,
}