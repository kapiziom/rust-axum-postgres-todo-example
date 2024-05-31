use std::sync::Arc;
use axum::extract::{State, Query};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::application::todos::models::todo_list_model::TodoListModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;
use crate::utils::pagination::Pager;

#[utoipa::path(
    get,
    path = "",
    params(
        ("pager" = Pager, Query, description = "Pager model")
    ),
    responses(
        (status = 200, description = "List all todos", body = [TodoListModel])
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_todos(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Query(pager): Query<Arc<Pager>>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let valid_sort_by = match pager.sort_by.as_str() {
        "id" | "title" | "date_created_utc" => pager.sort_by.to_string(),
        _ => "date_created_utc".to_string(),
    };

    let valid_sort_order = match pager.sort_order.to_uppercase().as_str() {
        "ASC" | "DESC" => pager.sort_order.to_uppercase(),
        _ => "ASC".to_string(),
    };

    let query_str = format!(
        r#"
        SELECT id, title, is_completed, date_created_utc
        FROM todos
        WHERE user_id = $1
        ORDER BY {} {}
        LIMIT $2 OFFSET $3
        "#,
        valid_sort_by,
        valid_sort_order
    );

    let todos = sqlx::query_as::<_, TodoListModel>(&query_str)
        .bind(user_id)
        .bind(pager.page_size)
        .bind(pager.offset)
        .fetch_all(&state.db)
        .await
        .map_err(|err| {
            let json_response = serde_json::json!({
                "status": "error",
                "message": err.to_string(),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
        })?;

    Ok((StatusCode::OK, Json(todos)))
}