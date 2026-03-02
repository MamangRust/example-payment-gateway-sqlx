use crate::state::AppState;
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use shared::{errors::ErrorResponse, utils::get_trace_id};
use std::sync::Arc;
use tracing::warn;

pub async fn rate_limit_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let rate_limiter = &app_state.rate_limit;
    let trace_id = get_trace_id();

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
                trace_id,
                status: "fail".to_string(),
                message: "Too many requests, please try again later".to_string(),
            }),
        ));
    }

    Ok(next.run(req).await)
}
