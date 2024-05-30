use std::sync::Arc;
use axum::extract::{Path, State, FromRequestParts};
use axum::http::{StatusCode, request::Parts};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use validator::Validate;
use crate::application::todos::models::update_todo_model::UpdateTodoModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;


#[utoipa::path(
    put,
    path = "/api/todos/{id}",
    params(
        ("id" = u32, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Update todo by ID", body = UpdateTodoModel)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(body): Json<UpdateTodoModel>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    if let Err(validation_errors) = body.validate() {
        let json_response = json!({
            "status": "error",
            "message": "Validation error",
            "errors": validation_errors
        });
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let result = sqlx::query!(
        "UPDATE todos \
        SET title = COALESCE($1, title), \
            description = COALESCE($2, description), \
            is_completed = COALESCE($3, is_completed) \
        WHERE id = $4 AND user_id = $5",
        body.title,
        body.description,
        body.is_completed,
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
        return Err((StatusCode::NOT_FOUND, Json(json_response)));
    }

    let json_response = json!({
        "status": "success",
        "message": "Todo updated"
    });

    Ok((StatusCode::OK, Json(json_response)))
}