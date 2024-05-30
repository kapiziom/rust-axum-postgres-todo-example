
use crate::application::users::commands::register::__path_register;
use crate::application::users::commands::authorize::__path_authorize;
use crate::application::users::queries::get_user::__path_get_user;
use crate::application::users::models::{
    user_model::UserModel,
    register_model::RegisterModel,
    login_model::LoginModel};
use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use utoipa::OpenApi;

use crate::application::users::commands::register::register;
use crate::application::users::commands::authorize::authorize;
use crate::application::users::queries::get_user::get_user;

use crate::server::state::AppState;

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/authorize", post(authorize))
        //.route("/api/users", get(get_users))
        .route("/api/users/me", get(get_user))
        .with_state(app_state)
}

#[derive(OpenApi)]
#[openapi(
    tags
    (
        (name = "users_router", description = "User data and auth routes")
    ),
    paths(authorize, register, get_user),
    components(schemas(LoginModel, RegisterModel, UserModel))
)]
pub(super) struct UserApi;