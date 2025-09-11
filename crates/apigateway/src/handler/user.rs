use crate::{
    middleware::{jwt, validate::SimpleValidatedJson},
    state::AppState,
};
use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde_json::json;
use shared::{
    abstract_trait::user::http::{
        command::DynUserCommandGrpcClient, query::DynUserQueryGrpcClient,
    },
    domain::{
        requests::user::{CreateUserRequest, FindAllUserRequest, UpdateUserRequest},
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynUserQueryGrpcClient>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynUserQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynUserQueryGrpcClient>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_active(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynUserQueryGrpcClient>,
    Query(params): Query<FindAllUserRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_trashed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/users",
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
    Extension(service): Extension<DynUserCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
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
    Extension(service): Extension<DynUserCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
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
    Extension(service): Extension<DynUserCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trashed(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
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
    Extension(service): Extension<DynUserCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynUserCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_permanent(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "User deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/users/restore-all",
    tag = "User",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed users restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_user_handler(
    Extension(service): Extension<DynUserCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All users restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/users/delete-all",
    tag = "User",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed users permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_user_handler(
    Extension(service): Extension<DynUserCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed users deleted permanently"
    })))
}

pub fn user_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/users", get(get_users))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/active", get(get_active_users))
        .route("/api/users/trashed", get(get_trashed_users))
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", put(update_user))
        .route("/api/users/trash/{id}", delete(trash_user_handler))
        .route("/api/users/restore/{id}", put(restore_user_handler))
        .route("/api/users/delete/{id}", delete(delete_user))
        .route("/api/users/restore-all", put(restore_all_user_handler))
        .route("/api/users/delete-all", delete(delete_all_user_handler))
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.user_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
