use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use crate::application::users::models::user_model::UserModel;
use crate::auth::AuthenticatedUser;
use crate::server::state::AppState;
use anyhow::Error;


#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Get current authorized user", body = UserModel)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser { user_id }: AuthenticatedUser)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let result: Result<_, Error> = async {
        let user = sqlx::query_as!(UserModel,
            r#"SELECT email, date_created_utc FROM users WHERE id = $1"#,
            user_id
        )
            .fetch_optional(&state.db)
            .await?;

        match user {
            Some(user) => Ok((StatusCode::OK, Json(json!(user)))),
            None => {
                let json_response = json!({
                    "status": "error",
                    "message": "User not found"
                });
                Ok((StatusCode::NOT_FOUND, Json(json_response)))
            }
        }
    }.await;

    result.map_err(|err| {
        let json_response = json!({
            "status": "error",
            "message": err.to_string()
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
    })
}