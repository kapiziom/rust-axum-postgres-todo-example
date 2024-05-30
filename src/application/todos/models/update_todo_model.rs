use serde::Deserialize;
use validator_derive::Validate;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodoModel {
    #[validate(length(min = 3, max = 250))]
    pub title: Option<String>,

    #[validate(length(min = 30, max = 1000))]
    pub description: Option<String>,

    pub is_completed: Option<bool>,
}