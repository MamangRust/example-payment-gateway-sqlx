use axum::{
    Extension, Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use shared::{abstract_trait::rate_limit::DynRateLimitMiddleware, errors::ErrorResponse};
use tracing::warn;

pub async fn rate_limit_middleware(
    Extension(rate_limiter): Extension<DynRateLimitMiddleware>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let key = format!("rate_limit:{client_ip}");
    let max_requests = 100;
    let window_seconds = 60;

    let (allowed, current) = rate_limiter
        .check_rate_limit(&key, max_requests, window_seconds)
        .await;

    if !allowed {
        warn!(
            "Rate limit exceeded for IP: {} (requests: {})",
            client_ip, current
        );
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(ErrorResponse {
                status: "fail".to_string(),
                message: "Too many requests, please try again later".to_string(),
            }),
        ));
    }

    Ok(next.run(req).await)
}
