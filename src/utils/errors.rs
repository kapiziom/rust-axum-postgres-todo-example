use serde_json::json;

pub fn errors_to_json(errors: validator::ValidationErrors) -> serde_json::Value {
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
