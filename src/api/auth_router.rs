
use crate::application::auth::commands::register::__path_register;
use crate::application::auth::commands::authorize::__path_authorize;
use crate::application::auth::models::{
    register_model::RegisterModel,
    login_model::LoginModel};
use std::sync::Arc;
use axum::Router;
use axum::routing::{post};
use utoipa::OpenApi;

use crate::application::auth::commands::register::register;
use crate::application::auth::commands::authorize::authorize;

use crate::server::state::AppState;

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/authorize", post(authorize))
        .with_state(app_state)
}

#[derive(OpenApi)]
#[openapi(
    tags
    (
        (name = "auth_router", description = "Auth routes")
    ),
    paths(authorize, register),
    components(schemas(LoginModel, RegisterModel))
)]
pub(super) struct AuthApi;