use std::sync::Arc;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::application::users::models::login_model::LoginModel;
use crate::server::state::AppState;
use jsonwebtoken::{encode, EncodingKey, Header};
use bcrypt::{verify};
use chrono::Utc;
use serde_json::json;
use crate::auth::jwt::generate_token;



#[utoipa::path(
    post,
    path = "/api/users/authorize",
    responses(
        (status = 200, description = "Login/authorization", body = LoginModel)
    )
)]
pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(mut body): Json<LoginModel>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        body.login
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|err| {
            let json_response = json!({ "status": "error", "message": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
        })?;

    if let Some(user) = user {
        if !verify(&body.password, &user.password_hash).unwrap() {
            let json_response = json!({ "status": "error", "message": "Invalid credentials" });
            return Ok((StatusCode::UNAUTHORIZED, Json(json_response)));
        }

        let token_data = generate_token(&user.id.to_string())
            .map_err(|err| {
                let json_response = json!({ "status": "error", "message": err.to_string() });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
            })?;

        let json_response = json!({ "status": "success", "token": token_data });
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let json_response = json!({ "status": "error", "message": "User not found" });
    Ok((StatusCode::NOT_FOUND, Json(json_response)))
}