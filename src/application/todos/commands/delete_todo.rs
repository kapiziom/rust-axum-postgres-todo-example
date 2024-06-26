use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;


#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = u32, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Delete todo by ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let result = sqlx::query!(
            "DELETE FROM todos WHERE id = $1 AND user_id = $2",
            id, user_id
        )
        .execute(&state.db)
        .await
        .map_err(|err| {
            let json_response = json!({
            "status": "error",
            "message": err.to_string()
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
        })?;

    if result.rows_affected() == 0 {
        let json_response = json!({
            "status": "error",
            "message": "Todo not found"
        });
        return Ok((StatusCode::NOT_FOUND, Json(json_response)))
    }

    let json_response = json!({
            "status": "success",
            "message": "Todo deleted"
        });

    Ok((StatusCode::OK, Json(json_response)))
}