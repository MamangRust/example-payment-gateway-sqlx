use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct ApiKey(pub String);

#[derive(Debug)]
pub struct ApiKeyError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiKeyError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
            "status": self.status.as_u16()
        }));
        (self.status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = ApiKeyError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(api_key_header) = parts.headers.get("x-api-key") {
            match api_key_header.to_str() {
                Ok(key) => {
                    if key.trim().is_empty() {
                        return Err(ApiKeyError {
                            status: StatusCode::BAD_REQUEST,
                            message: "API key cannot be empty".to_string(),
                        });
                    }
                    return Ok(ApiKey(key.to_string()));
                }
                Err(_) => {
                    return Err(ApiKeyError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Invalid API key format".to_string(),
                    });
                }
            }
        }

        if let Some(auth_header) = parts.headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if !token.trim().is_empty() {
                        return Ok(ApiKey(token.to_string()));
                    }
                }
            }
        }

        Err(ApiKeyError {
            status: StatusCode::UNAUTHORIZED,
            message: "Missing API key. Provide via 'x-api-key' header or 'Authorization: Bearer <token>' header".to_string(),
        })
    }
}
