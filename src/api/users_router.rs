use crate::application::users::queries::get_user::__path_get_user;
use crate::application::users::models::{
    user_model::UserModel};
use std::sync::Arc;
use axum::Router;
use axum::routing::{get};
use utoipa::OpenApi;

use crate::application::users::queries::get_user::get_user;

use crate::server::state::AppState;

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        //.route("/api/users", get(get_users))
        .route("/api/users/me", get(get_user))
        .with_state(app_state)
}

#[derive(OpenApi)]
#[openapi(
    tags
    (
        (name = "users_router", description = "User data routes")
    ),
    paths(get_user),
    components(schemas(UserModel))
)]
pub(super) struct UserApi;