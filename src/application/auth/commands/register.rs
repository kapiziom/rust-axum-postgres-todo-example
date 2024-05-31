use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Utc};
use serde_json::json;
use bcrypt::{hash_with_result, DEFAULT_COST};
use crate::application::auth::models::register_model::RegisterModel;
use crate::server::state::AppState;


#[utoipa::path(
    post,
    path = "/register",
    responses(
        (status = 200, description = "Register new account", body = RegisterModel)
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterModel>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    if body.password != body.repeat_password {
        let json_response = json!({
            "status": "error",
            "message": "Passwords do not match"
        });
        return Ok((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let user_exists = sqlx::query!("SELECT 1 as exists FROM users WHERE email = $1", body.login)
        .fetch_optional(&state.db)
        .await
        .map_err(|err| {
            let json_response = json!({
                "status": "error",
                "message": err.to_string()
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
        })?;

    if user_exists.is_some() {
        let json_response = json!({
            "status": "error",
            "message": "User already exists"
        });
        return Ok((StatusCode::CONFLICT, Json(json_response)));
    }

    let hash = bcrypt::hash(body.password, DEFAULT_COST).unwrap();
    let date_created_utc = Utc::now();

    let new_user =
        sqlx::query!(
            "INSERT INTO users (email, password_hash, date_created_utc) VALUES ($1, $2, $3) RETURNING id",
            body.login,
            hash,
            Utc::now())
            .fetch_one(&state.db)
            .await
            .map_err(|err| {
                let json_response = json!({
            "status": "error",
            "message": err.to_string()
        });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
            })?;

    let json_response = json!({
        "status": "success",
        "message": "User created successfully",
        "user_id": new_user.id
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}