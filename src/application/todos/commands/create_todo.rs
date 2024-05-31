use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use crate::application::todos::models::create_todo_model::CreateTodoModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;
use crate::utils::validation::validate_body;


#[utoipa::path(
    post,
    path = "",
    responses(
        (status = 200, description = "Create new Todo", body = CreateTodoModel)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(body): Json<CreateTodoModel>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    validate_body(&body).await?;

    let result = sqlx::query!(
        "INSERT INTO todos (user_id, title, description) VALUES ($1, $2, $3) RETURNING id",
        user_id,
        body.title,
        body.description,
    )
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
        "message": "Todo created successfully",
        "todo_id": result.id,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}