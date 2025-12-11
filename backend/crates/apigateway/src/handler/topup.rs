use crate::{
    middleware::{
        jwt, rate_limit::rate_limit_middleware, session::session_middleware,
        validate::SimpleValidatedJson,
    },
    state::AppState,
};
use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde_json::json;
use shared::{
    abstract_trait::{session::DynSessionMiddleware, topup::http::DynTopupGrpcClientService},
    domain::{
        requests::{
            topup::{
                CreateTopupRequest, FindAllTopups, FindAllTopupsByCardNumber, MonthTopupStatus,
                MonthTopupStatusCardNumber, UpdateTopupRequest, YearMonthMethod,
                YearTopupStatusCardNumber,
            },
            withdraw::YearQuery,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TopupMonthAmountResponse, TopupMonthMethodResponse,
            TopupResponse, TopupResponseDeleteAt, TopupResponseMonthStatusFailed,
            TopupResponseMonthStatusSuccess, TopupResponseYearStatusFailed,
            TopupResponseYearStatusSuccess, TopupYearlyAmountResponse, TopupYearlyMethodResponse,
        },
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/topups",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(FindAllTopups),
    responses(
        (status = 200, description = "List of topups", body = ApiResponsePagination<Vec<TopupResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_topups(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, HttpError> {
    match service.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/by-card",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(FindAllTopupsByCardNumber),
    responses(
        (status = 200, description = "List of topups by card number", body = ApiResponsePagination<Vec<TopupResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_topups_by_card_number(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<FindAllTopupsByCardNumber>,
) -> Result<impl IntoResponse, HttpError> {
    match service.find_all_by_card_number(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/active",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(FindAllTopups),
    responses(
        (status = 200, description = "List of active topups", body = ApiResponsePagination<Vec<TopupResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_topups(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, HttpError> {
    match service.find_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/trashed",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(FindAllTopups),
    responses(
        (status = 200, description = "List of soft-deleted topups", body = ApiResponsePagination<Vec<TopupResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_topups(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, HttpError> {
    match service.find_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/{id}",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Topup ID")),
    responses(
        (status = 200, description = "Topup details", body = ApiResponse<TopupResponse>),
        (status = 404, description = "Topup not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_topup(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    match service.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/create",
    tag = "Topup",
    security(("bearer_auth" = [])),
    request_body = CreateTopupRequest,
    responses(
        (status = 201, description = "Topup created", body = ApiResponse<TopupResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_topup(
    Extension(service): Extension<DynTopupGrpcClientService>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTopupRequest>,
) -> Result<impl IntoResponse, HttpError> {
    match service.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/update/{id}",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Topup ID")),
    request_body = UpdateTopupRequest,
    responses(
        (status = 200, description = "Topup updated", body = ApiResponse<TopupResponse>),
        (status = 404, description = "Topup not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_topup(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTopupRequest>,
) -> Result<impl IntoResponse, HttpError> {
    body.topup_id = Some(id);
    match service.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/trash/{id}",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Topup ID")),
    responses(
        (status = 200, description = "Topup soft-deleted", body = ApiResponse<TopupResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_topup_handler(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.trashed(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/restore/{id}",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Topup ID")),
    responses(
        (status = 200, description = "Topup restored", body = ApiResponse<TopupResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_topup_handler(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/topups/delete/{id}",
    tag = "Topup",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Topup ID")),
    responses(
        (status = 200, description = "Topup permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_topup(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Topup deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/restore-all",
    tag = "Topup",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed topups restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_topup_handler(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All topups restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/topups/delete-all",
    tag = "Topup",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed topups permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_topup_handler(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.delete_all_permanent().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed topups deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly topup amount", body = ApiResponse<Vec<TopupMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_amounts(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_monthly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly topup amount", body = ApiResponse<Vec<TopupYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_amounts(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly topup method", body = ApiResponse<Vec<TopupMonthMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_methods(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_monthly_methods(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly topup method", body = ApiResponse<Vec<TopupYearlyMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_methods(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_methods(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/success/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(MonthTopupStatus),
    responses(
        (status = 200, description = "Monthly successful topup status", body = ApiResponse<Vec<TopupResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_topup_status_success(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<MonthTopupStatus>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_month_status_success(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/success/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly successful topup status", body = ApiResponse<Vec<TopupResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_success(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_status_success(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/failed/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(MonthTopupStatus),
    responses(
        (status = 200, description = "Monthly failed topup status", body = ApiResponse<Vec<TopupResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_topup_status_failed(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<MonthTopupStatus>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_month_status_failed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/failed/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly failed topup status", body = ApiResponse<Vec<TopupResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_failed(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_status_failed(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/monthly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthMethod),
    responses(
        (status = 200, description = "Monthly topup amount by card", body = ApiResponse<Vec<TopupMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_amounts_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearMonthMethod>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_monthly_amounts_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/yearly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthMethod),
    responses(
        (status = 200, description = "Yearly topup amount by card", body = ApiResponse<Vec<TopupYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_amounts_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearMonthMethod>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_amounts_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/monthly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthMethod),
    responses(
        (status = 200, description = "Monthly topup method by card", body = ApiResponse<Vec<TopupMonthMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_methods_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearMonthMethod>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_monthly_methods_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/yearly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthMethod),
    responses(
        (status = 200, description = "Yearly topup method by card", body = ApiResponse<Vec<TopupYearlyMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_methods_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearMonthMethod>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_methods_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/success/monthly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthTopupStatusCardNumber),
    responses(
        (status = 200, description = "Monthly successful topup status by card", body = ApiResponse<Vec<TopupResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_topup_status_success_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<MonthTopupStatusCardNumber>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_month_status_success_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/success/yearly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearTopupStatusCardNumber),
    responses(
        (status = 200, description = "Yearly successful topup status by card", body = ApiResponse<Vec<TopupResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_success_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearTopupStatusCardNumber>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_status_success_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/failed/monthly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthTopupStatusCardNumber),
    responses(
        (status = 200, description = "Monthly failed topup status by card", body = ApiResponse<Vec<TopupResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_topup_status_failed_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<MonthTopupStatusCardNumber>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_month_status_failed_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/failed/yearly/by-card",
    tag = "Topup Stats By Card",
    security(("bearer_auth" = [])),
    params(YearTopupStatusCardNumber),
    responses(
        (status = 200, description = "Yearly failed topup status by card", body = ApiResponse<Vec<TopupResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_failed_by_card(
    Extension(service): Extension<DynTopupGrpcClientService>,
    Query(params): Query<YearTopupStatusCardNumber>,
    Extension(user_id): Extension<i32>,
    Extension(session): Extension<DynSessionMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
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

    match service.get_yearly_status_failed_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

pub fn topup_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/topups", get(get_topups))
        .route("/api/topups/create", post(create_topup))
        .route("/api/topups/update/{id}", post(update_topup))
        .route("/api/topups/by-card", get(get_topups_by_card_number))
        .route("/api/topups/active", get(get_active_topups))
        .route("/api/topups/trashed", get(get_trashed_topups))
        .route("/api/topups/{id}", get(get_topup))
        .route("/api/topups/trash/{id}", post(trash_topup_handler))
        .route("/api/topups/restore/{id}", post(restore_topup_handler))
        .route("/api/topups/delete/{id}", delete(delete_topup))
        .route("/api/topups/restore-all", post(restore_all_topup_handler))
        .route("/api/topups/delete-all", post(delete_all_topup_handler))
        .route(
            "/api/topups/stats/amount/monthly",
            get(get_monthly_topup_amounts),
        )
        .route(
            "/api/topups/stats/amount/yearly",
            get(get_yearly_topup_amounts),
        )
        .route(
            "/api/topups/stats/method/monthly",
            get(get_monthly_topup_methods),
        )
        .route(
            "/api/topups/stats/method/yearly",
            get(get_yearly_topup_methods),
        )
        .route(
            "/api/topups/stats/status/success/monthly",
            get(get_month_topup_status_success),
        )
        .route(
            "/api/topups/stats/status/success/yearly",
            get(get_yearly_topup_status_success),
        )
        .route(
            "/api/topups/stats/status/failed/monthly",
            get(get_month_topup_status_failed),
        )
        .route(
            "/api/topups/stats/status/failed/yearly",
            get(get_yearly_topup_status_failed),
        )
        .route(
            "/api/topups/stats/amount/monthly/by-card",
            get(get_monthly_topup_amounts_by_card),
        )
        .route(
            "/api/topups/stats/amount/yearly/by-card",
            get(get_yearly_topup_amounts_by_card),
        )
        .route(
            "/api/topups/stats/method/monthly/by-card",
            get(get_monthly_topup_methods_by_card),
        )
        .route(
            "/api/topups/stats/method/yearly/by-card",
            get(get_yearly_topup_methods_by_card),
        )
        .route(
            "/api/topups/stats/status/success/monthly/by-card",
            get(get_month_topup_status_success_by_card),
        )
        .route(
            "/api/topups/stats/status/success/yearly/by-card",
            get(get_yearly_topup_status_success_by_card),
        )
        .route(
            "/api/topups/stats/status/failed/monthly/by-card",
            get(get_month_topup_status_failed_by_card),
        )
        .route(
            "/api/topups/stats/status/failed/yearly/by-card",
            get(get_yearly_topup_status_failed_by_card),
        )
        .route_layer(middleware::from_fn(session_middleware))
        .route_layer(middleware::from_fn(jwt::auth))
        .route_layer(middleware::from_fn(rate_limit_middleware))
        .layer(Extension(app_state.di_container.topup_clients.clone()))
        .layer(Extension(app_state.di_container.role_clients.clone()))
        .layer(Extension(app_state.session.clone()))
        .layer(Extension(app_state.rate_limit.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
