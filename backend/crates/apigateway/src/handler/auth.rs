use crate::{
    middleware::{jwt, rate_limit::rate_limit_middleware, validate::SimpleValidatedJson},
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use shared::{
    domain::{
        requests::{
            auth::{AuthRequest, RegisterRequest},
            refresh_token::RefreshTokenRequest,
        },
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

    axum::Json(serde_json::json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = AuthRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Auth"
)]
pub async fn login_user_handler(
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<AuthRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let auth_client = &app_state.di_container.auth_clients;

    let response = auth_client.login(&body).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Auth"
)]
pub async fn register_user_handler(
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<RegisterRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let auth_client = &app_state.di_container.auth_clients;

    let response = auth_client.register(&body).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Get Me user", body = ApiResponse<UserResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Auth",
)]
pub async fn get_me_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let auth_client = &app_state.di_container.auth_clients;

    let response = auth_client.get_me(user_id).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh-token",
    request_body(content = String, description = "Refresh token", content_type = "application/json"),
    responses(
        (status = 200, description = "Token refreshed", body = ApiResponse<TokenResponse>),
        (status = 401, description = "Invalid or expired refresh token")
    ),
    tag = "Auth"
)]
pub async fn refresh_token_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let auth_client = &app_state.di_container.auth_clients;

    let response = auth_client.refresh_token(&req.refresh_token).await?;
    Ok((StatusCode::OK, Json(response)))
}

pub fn auth_routes(state: Arc<AppState>) -> OpenApiRouter {
    let public_routes = OpenApiRouter::new()
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(state.clone());

    let private_routes = OpenApiRouter::new()
        .route("/api/auth/me", get(get_me_handler))
        .route("/api/auth/refresh-token", post(refresh_token_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), jwt::auth))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .with_state(state);
    public_routes.merge(private_routes)
}
