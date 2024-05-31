use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::api::auth_router::auth_routes;
use crate::api::todos_router::todo_routes;
use crate::api::users_router::user_routes;
use crate::server::state::AppState;

use super::{ApiDoc, health_checker_handler};

pub fn create_router(app_state: Arc<AppState>, environment: &str) -> Router {
    let mut router = Router::new()
        .route("/api/health", get(health_checker_handler))
        .with_state(app_state.clone())
        .merge(auth_routes(app_state.clone()))
        .merge(user_routes(app_state.clone()))
        .merge(todo_routes(app_state.clone()));

    if environment == "development" {
        router = router
            .merge(SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()));
    }

    router
}