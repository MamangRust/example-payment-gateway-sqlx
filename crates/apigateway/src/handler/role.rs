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
    abstract_trait::role::http::{
        command::DynRoleCommandGrpcClient, query::DynRoleQueryGrpcClient,
    },
    domain::{
        requests::role::{CreateRoleRequest, FindAllRoles, UpdateRoleRequest},
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynRoleQueryGrpcClient>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynRoleQueryGrpcClient>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_active(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynRoleQueryGrpcClient>,
    Query(params): Query<FindAllRoles>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_trashed(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynRoleQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynRoleQueryGrpcClient>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_user_id(user_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/roles",
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
    Extension(service): Extension<DynRoleCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateRoleRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/roles/{id}",
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
    Extension(service): Extension<DynRoleCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateRoleRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/roles/trash/{id}",
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
    Extension(service): Extension<DynRoleCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trash(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
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
    Extension(service): Extension<DynRoleCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynRoleCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Role deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/roles/restore-all",
    tag = "Role",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed roles restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_role_handler(
    Extension(service): Extension<DynRoleCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All roles restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/roles/delete-all",
    tag = "Role",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed roles permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_role_handler(
    Extension(service): Extension<DynRoleCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed roles deleted permanently"
    })))
}

pub fn role_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/roles", get(get_roles))
        .route("/api/roles/active", get(get_active_roles))
        .route("/api/roles/trashed", get(get_trashed_roles))
        .route("/api/roles/:id", get(get_role))
        .route("/api/roles/user/:user_id", get(get_roles_by_user_id))
        .route("/api/roles", post(create_role))
        .route("/api/roles/:id", put(update_role))
        .route("/api/roles/trash/:id", delete(trash_role_handler))
        .route("/api/roles/restore/:id", put(restore_role_handler))
        .route("/api/roles/delete/:id", delete(delete_role))
        .route("/api/roles/restore-all", put(restore_all_role_handler))
        .route("/api/roles/delete-all", delete(delete_all_role_handler))
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.role_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
