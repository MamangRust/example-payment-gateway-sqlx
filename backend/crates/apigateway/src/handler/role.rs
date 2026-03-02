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
        requests::role::{CreateRoleRequest, FindAllRoles, UpdateRoleRequest},
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/roles",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(FindAllRoles),
    responses(
        (status = 200, description = "List of roles", body = ApiResponsePagination<Vec<RoleResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_roles(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

    match role_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/active",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(FindAllRoles),
    responses(
        (status = 200, description = "List of active roles", body = ApiResponsePagination<Vec<RoleResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_roles(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

    match role_client.find_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/trashed",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(FindAllRoles),
    responses(
        (status = 200, description = "List of soft-deleted roles", body = ApiResponsePagination<Vec<RoleResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_roles(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

    match role_client.find_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/{id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role details", body = ApiResponse<RoleResponse>),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_role(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

    match role_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/user/{user_id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("user_id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "List of roles for user", body = ApiResponse<Vec<RoleResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_roles_by_user_id(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

    match role_client.find_by_user_id(user_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/create",
    tag = "Role",
    security(("bearer_auth" = [])),
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created", body = ApiResponse<RoleResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_role(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateRoleRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/update/{id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Role ID")),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated", body = ApiResponse<RoleResponse>),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_role(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,

    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateRoleRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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
    match role_client.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/trashed/{id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role soft-deleted", body = ApiResponse<RoleResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_role_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.trash(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/restore/{id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role restored", body = ApiResponse<RoleResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_role_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/roles/delete/{id}",
    tag = "Role",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Role ID")),
    responses(
        (status = 200, description = "Role permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_role(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.delete(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Role deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/restore-all",
    tag = "Role",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed roles restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_role_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All roles restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/roles/delete-all",
    tag = "Role",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed roles permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_role_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let role_client = &app_state.di_container.role_clients;

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

    match role_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed roles deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

pub fn role_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/roles", get(get_roles))
        .route("/api/roles/active", get(get_active_roles))
        .route("/api/roles/trashed", get(get_trashed_roles))
        .route("/api/roles/{id}", get(get_role))
        .route("/api/roles/user/{user_id}", get(get_roles_by_user_id))
        .route("/api/roles/create", post(create_role))
        .route("/api/roles/update/{id}", post(update_role))
        .route("/api/roles/trashed/{id}", post(trash_role_handler))
        .route("/api/roles/restore/{id}", post(restore_role_handler))
        .route("/api/roles/delete/{id}", delete(delete_role))
        .route("/api/roles/restore-all", post(restore_all_role_handler))
        .route("/api/roles/delete-all", post(delete_all_role_handler))
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
