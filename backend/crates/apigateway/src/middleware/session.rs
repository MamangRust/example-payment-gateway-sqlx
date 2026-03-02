use crate::state::AppState;
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use chrono::Duration;
use shared::{domain::responses::Session, errors::ErrorResponse, utils::get_trace_id};
use std::sync::Arc;

pub async fn session_middleware(
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let role_client = &app_state.di_container.role_clients;
    let session_service = &app_state.session;
    let trace_id = get_trace_id();

    let user_id = match req.extensions().get::<i32>() {
        Some(id) => *id,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    trace_id: trace_id.clone(),
                    status: "fail".to_string(),
                    message: "Missing user_id in request context".to_string(),
                }),
            ));
        }
    };

    let roles = match role_client.find_by_user_id(user_id).await {
        Ok(resp) => resp.data.into_iter().map(|r| r.name).collect(),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    trace_id: trace_id.clone(),
                    status: "fail".to_string(),
                    message: "Failed to fetch roles".to_string(),
                }),
            ));
        }
    };

    let session = Session {
        user_id: user_id.to_string(),
        roles,
    };

    let key = format!("session:{user_id}");

    session_service
        .create_session(&key, &session, Duration::minutes(30))
        .await;

    req.extensions_mut().insert(session.clone());

    Ok(next.run(req).await)
}
