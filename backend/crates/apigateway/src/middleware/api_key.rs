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
        if let Some(key) = parts
            .headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            return Ok(ApiKey(key.to_string()));
        }

        Err(ApiKeyError {
            status: StatusCode::UNAUTHORIZED,
            message: "Missing API key. Provide via 'x-api-key''".to_string(),
        })
    }
}
