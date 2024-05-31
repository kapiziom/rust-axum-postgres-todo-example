use axum::{Json, response::IntoResponse};
use utoipa::{Modify, OpenApi, openapi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

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
    nest(
        (path = "/api/todos", api = todos_router::TodoApi),
        (path = "/api/users", api = users_router::UserApi),
        (path = "/api/auth", api = auth_router::AuthApi)
    )
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

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}