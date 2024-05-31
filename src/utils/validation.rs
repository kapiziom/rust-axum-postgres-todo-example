use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use validator::Validate;

pub async fn validate_body<T: Validate>(body: &T) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if let Err(errors) = body.validate() {
        let json_response = json!({
            "status": "error",
            "message": "Validation error",
            "errors": errors_to_json(errors)
        });
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }
    Ok(())
}


fn errors_to_json(errors: validator::ValidationErrors) -> serde_json::Value {
    let mut errors_json = json!({});
    for (field, errors) in errors.field_errors() {
        let mut field_errors = vec![];
        for error in errors {
            if let Some(message) = &error.message {
                field_errors.push(message.to_string());
            }
        }
        errors_json[field] = json!(field_errors);
    }
    errors_json
}