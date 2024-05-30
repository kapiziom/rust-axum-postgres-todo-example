use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let json_response = serde_json::json!({
        "status": "error",
        "message": "method not implemented",
    });

    Ok((StatusCode::NOT_IMPLEMENTED, Json(json_response)))
}