use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterModel {
    pub login: String,
    pub password: String,
    pub repeat_password: String
}