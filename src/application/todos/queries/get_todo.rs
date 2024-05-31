use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::application::todos::models::todo_details_model::TodoDetailsModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;


#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = u32, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Get a todo by ID", body = TodoDetailsModel)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let todo = sqlx::query_as!(TodoDetailsModel,
            r#"SELECT id, title, description, is_completed, date_created_utc, date_modified_utc
                FROM todos WHERE id = $1 AND user_id = $2"#,
            id, &user_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|err| {
            let json_response = json!({ "status": "error", "message": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
        })?;

    match todo {
        Some(todo) => Ok((StatusCode::OK, Json(json!(todo)))),
        None => {
            let json_response = json!({
                "status": "error",
                "message": "Todo not found"
            });
            Err((StatusCode::NOT_FOUND, Json(json_response)))
        }
    }
}