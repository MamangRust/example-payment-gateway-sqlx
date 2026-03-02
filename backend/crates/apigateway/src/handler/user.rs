use crate::middleware::session::session_middleware;
use crate::{
    middleware::{
        circuit_breaker::circuit_breaker_middleware, jwt,
        request_limiter::request_limiter_middleware, validate::SimpleValidatedJson,
    },
    state::AppState,
};
use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde_json::json;
use shared::{
    domain::{
        requests::user::{CreateUserRequest, FindAllUserRequest, UpdateUserRequest},
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "User",
    security(("bearer_auth" = [])),
    params(FindAllUserRequest),
    responses(
        (status = 200, description = "List of users", body = ApiResponsePagination<Vec<UserResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    match user_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "User",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User details", body = ApiResponse<UserResponse>),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    match user_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/active",
    tag = "User",
    security(("bearer_auth" = [])),
    params(FindAllUserRequest),
    responses(
        (status = 200, description = "List of active users", body = ApiResponsePagination<Vec<UserResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    match user_client.find_by_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/trashed",
    tag = "User",
    security(("bearer_auth" = [])),
    params(FindAllUserRequest),
    responses(
        (status = 200, description = "List of soft-deleted users", body = ApiResponsePagination<Vec<UserResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_users(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    match user_client.find_by_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/create",
    tag = "User",
    security(("bearer_auth" = [])),
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = ApiResponse<UserResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,

    SimpleValidatedJson(body): SimpleValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/update/{id}",
    tag = "User",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "User ID")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = ApiResponse<UserResponse>),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    body.id = Some(id);
    match user_client.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/trash/{id}",
    tag = "User",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User soft-deleted", body = ApiResponse<UserResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_user_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.trashed(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/restore/{id}",
    tag = "User",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User restored", body = ApiResponse<UserResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_user_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/delete/{id}",
    tag = "User",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "User permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "User deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/restore-all",
    tag = "User",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed users restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_user_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All users restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/delete-all",
    tag = "User",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed users permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_user_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let user_client = &app_state.di_container.user_clients;

    let session = &app_state.session;

    let key = format!("session:{user_id}");

    let current_session = session
        .get_session(&key)
        .await
        .ok_or_else(|| HttpError::Unauthorized("Session expired or not found".to_string()))?;

    if !current_session
        .roles
        .iter()
        .any(|r| r == "ROLE_ADMIN" || r == "ROLE_MODERATOR")
    {
        return Err(HttpError::Forbidden(
            "Access denied. Required role: ADMIN or MODERATOR".to_string(),
        ));
    }

    match user_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed users deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

pub fn user_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/users", get(get_users))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/active", get(get_active_users))
        .route("/api/users/trashed", get(get_trashed_users))
        .route("/api/users/create", post(create_user))
        .route("/api/users/update/{id}", post(update_user))
        .route("/api/users/trash/{id}", post(trash_user_handler))
        .route("/api/users/restore/{id}", post(restore_user_handler))
        .route("/api/users/delete/{id}", delete(delete_user))
        .route("/api/users/restore-all", post(restore_all_user_handler))
        .route("/api/users/delete-all", post(delete_all_user_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            session_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(state.clone(), jwt::auth))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            circuit_breaker_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            request_limiter_middleware,
        ))
        .with_state(state)
}
