use axum::{Json, response::IntoResponse};
use utoipa::{Modify, OpenApi, openapi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use crate::utils::pagination::Pager;

pub mod router;
pub mod auth_router;
pub mod users_router;
pub mod todos_router;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Ok!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    nest(
        (path = "/api/todos", api = todos_router::TodoApi),
        (path = "/api/users", api = users_router::UserApi),
        (path = "/api/auth", api = auth_router::AuthApi)
    ),
    components(schemas(Pager))
)]
struct ApiDoc;

struct SecurityAddon;


impl Modify for SecurityAddon {

    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer.into())
                        .bearer_format("JWT")
                        .build()
                ),
            );
        }
    }
}