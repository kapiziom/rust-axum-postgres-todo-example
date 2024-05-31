use std::sync::Arc;
use axum::extract::{Path, State, FromRequestParts};
use axum::http::{StatusCode, request::Parts};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::application::todos::models::update_todo_model::UpdateTodoModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;
use crate::utils::validation::validate_body;


#[utoipa::path(
    put,
    path = "/{id}",
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
    validate_body(&body).await?;

    let result = sqlx::query!(
        "UPDATE todos \
        SET title = COALESCE($1, title), \
            description = COALESCE($2, description), \
            is_completed = COALESCE($3, is_completed), \
            date_modified_utc = COALESCE($4, date_modified_utc) \
        WHERE id = $5 AND user_id = $6",
        body.title,
        body.description,
        body.is_completed,
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
        return Err((StatusCode::NOT_FOUND, Json(json_response)));
    }

    let json_response = json!({
        "status": "success",
        "message": "Todo updated"
    });

    Ok((StatusCode::OK, Json(json_response)))
}