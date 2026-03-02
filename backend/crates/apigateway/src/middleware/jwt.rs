use crate::state::AppState;
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use shared::{errors::ErrorResponse, utils::get_trace_id};
use std::sync::Arc;

pub async fn auth(
    cookie_jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let jwt = &app_state.jwt_config;
    let trace_id = get_trace_id();

    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(str::to_owned))
        });

    let token = match token {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    trace_id: trace_id.clone(),
                    status: "fail".to_string(),
                    message: "You are not logged in, please provide token".to_string(),
                }),
            ));
        }
    };

    let user_id = match jwt.verify_token(&token, "access") {
        Ok(id) => id as i32,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    trace_id: trace_id.clone(),
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                }),
            ));
        }
    };

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
