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

pub async fn request_limiter_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let limiter = &app_state.request_limiter;
    let trace_id = get_trace_id();

    match limiter.semaphore.try_acquire() {
        Ok(_permit) => {
            let response = next.run(req).await;
            Ok(response)
        }
        Err(_) => {
            let available = limiter.available_permits();
            warn!(
                "⚠️  Request limiter: Too many concurrent requests (limit: {}, available: {})",
                limiter.max_concurrent, available
            );

            Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(ErrorResponse {
                    trace_id,
                    status: "error".to_string(),
                    message: format!(
                        "Too many concurrent requests. Server is handling {} requests. Please try again later.",
                        limiter.max_concurrent
                    ),
                }),
            ))
        }
    }
}
