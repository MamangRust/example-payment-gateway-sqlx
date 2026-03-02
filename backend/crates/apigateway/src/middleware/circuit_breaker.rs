use crate::state::AppState;
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use shared::{errors::ErrorResponse, utils::get_trace_id};
use std::sync::Arc;
use tracing::warn;

pub async fn circuit_breaker_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let circuit_breaker = &app_state.circuit_breaker;
    let trace_id = get_trace_id();

    if !circuit_breaker.should_allow_request().await {
        warn!("🔴 Request rejected by circuit breaker");
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                status: "error".to_string(),
                message: "Service temporarily unavailable due to high error rate. Please try again later.".to_string(),
                trace_id: trace_id.clone()
            }),
        ));
    }

    let response = next.run(req).await;

    let status = response.status();

    if status.is_server_error() {
        circuit_breaker.record_failure();
        warn!("❌ Request failed with status: {}", status);
    } else if status.is_success() {
        circuit_breaker.record_success();
    }

    Ok(response)
}
