use std::fmt::Write;
use validator::ValidationErrors;

pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut result = String::new();

    for (field, field_errors) in errors.field_errors() {
        for err in field_errors {
            let message = err
                .message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| match err.code.as_ref() {
                    "email" => "invalid email format".to_string(),
                    "url" => "invalid URL format".to_string(),
                    "length" => "invalid length".to_string(),
                    "range" => "value out of range".to_string(),
                    "required" => "required".to_string(),
                    "custom" => "custom validation failed".to_string(),
                    _ => "invalid value".to_string(),
                });

            writeln!(&mut result, "{field}: {message}").unwrap();
        }
    }

    if result.is_empty() {
        "Validation failed".to_string()
    } else {
        result.trim().to_string()
    }
}
