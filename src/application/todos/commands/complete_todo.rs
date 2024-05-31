use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;

#[utoipa::path(
    patch,
    path = "/{id}/complete",
    params(
        ("id" = u32, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Complete todo by id")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn complete_todo(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let result = sqlx::query!(
        "UPDATE todos \
        SET is_completed = COALESCE($1, is_completed), \
            date_modified_utc = COALESCE($2, date_modified_utc) \
        WHERE id = $3 AND user_id = $4",
        true,
        chrono::Utc::now(),
        id,
        user_id
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
        return Ok((StatusCode::NOT_FOUND, Json(json_response)));
    }

    let json_response = json!({
        "status": "success",
        "message": "Todo completed"
    });

    Ok((StatusCode::OK, Json(json_response)))
}