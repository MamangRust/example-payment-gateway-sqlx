use crate::{
    middleware::{
        circuit_breaker::circuit_breaker_middleware, jwt, rate_limit::rate_limit_middleware,
        request_limiter::request_limiter_middleware, session::session_middleware,
        validate::SimpleValidatedJson,
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
        requests::withdraw::{
            CreateWithdrawRequest, FindAllWithdrawCardNumber, FindAllWithdraws,
            MonthStatusWithdraw, MonthStatusWithdrawCardNumber, UpdateWithdrawRequest,
            YearMonthCardNumber, YearQuery, YearStatusWithdrawCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawMonthlyAmountResponse, WithdrawResponse,
            WithdrawResponseDeleteAt, WithdrawResponseMonthStatusFailed,
            WithdrawResponseMonthStatusSuccess, WithdrawResponseYearStatusFailed,
            WithdrawResponseYearStatusSuccess, WithdrawYearlyAmountResponse,
        },
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/withdraws",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(FindAllWithdraws),
    responses(
        (status = 200, description = "List of withdraws", body = ApiResponsePagination<Vec<WithdrawResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_withdraws(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/by-card",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(FindAllWithdrawCardNumber),
    responses(
        (status = 200, description = "List of withdraws by card number", body = ApiResponsePagination<Vec<WithdrawResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_withdraws_by_card_number(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllWithdrawCardNumber>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.find_all_by_card_number(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/{id}",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Withdraw ID")),
    responses(
        (status = 200, description = "Withdraw details", body = ApiResponse<WithdrawResponse>),
        (status = 404, description = "Withdraw not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_withdraw(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/active",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(FindAllWithdraws),
    responses(
        (status = 200, description = "List of active withdraws", body = ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_withdraws(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.find_by_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/trashed",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(FindAllWithdraws),
    responses(
        (status = 200, description = "List of soft-deleted withdraws", body = ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_withdraws(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.find_by_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/withdraws/create",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    request_body = CreateWithdrawRequest,
    responses(
        (status = 201, description = "Withdraw created", body = ApiResponse<WithdrawResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_withdraw(
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateWithdrawRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    match withdraw_client.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    put,
    path = "/api/withdraws/update/{id}",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Withdraw ID")),
    request_body = UpdateWithdrawRequest,
    responses(
        (status = 200, description = "Withdraw updated", body = ApiResponse<WithdrawResponse>),
        (status = 404, description = "Withdraw not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_withdraw(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateWithdrawRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

    body.withdraw_id = Some(id);
    match withdraw_client.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/withdraws/trash/{id}",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Withdraw ID")),
    responses(
        (status = 200, description = "Withdraw soft-deleted", body = ApiResponse<WithdrawResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_withdraw_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.trashed_withdraw(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/withdraws/restore/{id}",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Withdraw ID")),
    responses(
        (status = 200, description = "Withdraw restored", body = ApiResponse<WithdrawResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_withdraw_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/withdraws/delete/{id}",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Withdraw ID")),
    responses(
        (status = 200, description = "Withdraw permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_withdraw(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Withdraw deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/withdraws/restore-all",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed withdraws restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_withdraw_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All withdraws restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/withdraws/delete-all",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed withdraws permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_withdraw_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed withdraws deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/monthly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly withdraw amount", body = ApiResponse<Vec<WithdrawMonthlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_withdraws(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_monthly_withdraws(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly withdraw amount", body = ApiResponse<Vec<WithdrawYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_withdraws(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_yearly_withdraws(query.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/success/monthly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusWithdraw),
    responses(
        (status = 200, description = "Monthly successful withdraw status", body = ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusWithdraw>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_month_status_success(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/success/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly successful withdraw status", body = ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_yearly_status_success(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/failed/monthly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusWithdraw),
    responses(
        (status = 200, description = "Monthly failed withdraw status", body = ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusWithdraw>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_month_status_failed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/failed/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly failed withdraw status", body = ApiResponse<Vec<WithdrawResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_yearly_status_failed(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/monthly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthCardNumber),
    responses(
        (status = 200, description = "Monthly withdraw amount by card", body = ApiResponse<Vec<WithdrawMonthlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_by_card_number(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearMonthCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_monthly_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/yearly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(YearMonthCardNumber),
    responses(
        (status = 200, description = "Yearly withdraw amount by card", body = ApiResponse<Vec<WithdrawYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_by_card_number(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearMonthCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client.get_yearly_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/success/monthly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusWithdrawCardNumber),
    responses(
        (status = 200, description = "Monthly successful withdraw status by card", body = ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusWithdrawCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client
        .get_month_status_success_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/success/yearly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusWithdrawCardNumber),
    responses(
        (status = 200, description = "Yearly successful withdraw status by card", body = ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusWithdrawCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client
        .get_yearly_status_success_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/failed/monthly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusWithdrawCardNumber),
    responses(
        (status = 200, description = "Monthly failed withdraw status by card", body = ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusWithdrawCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client
        .get_month_status_failed_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/failed/yearly/by-card",
    tag = "Withdraw Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusWithdrawCardNumber),
    responses(
        (status = 200, description = "Yearly failed withdraw status by card", body = ApiResponse<Vec<WithdrawResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusWithdrawCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let withdraw_client = &app_state.di_container.withdraw_clients;

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

    match withdraw_client
        .get_yearly_status_failed_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

pub fn withdraw_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/withdraws", get(get_withdraws))
        .route("/api/withdraws/by-card", get(get_withdraws_by_card_number))
        .route("/api/withdraws/{id}", get(get_withdraw))
        .route("/api/withdraws/active", get(get_active_withdraws))
        .route("/api/withdraws/trashed", get(get_trashed_withdraws))
        .route("/api/withdraws/create", post(create_withdraw))
        .route("/api/withdraws/update/{id}", post(update_withdraw))
        .route("/api/withdraws/trash/{id}", post(trash_withdraw_handler))
        .route(
            "/api/withdraws/restore/{id}",
            post(restore_withdraw_handler),
        )
        .route("/api/withdraws/delete/{id}", delete(delete_withdraw))
        .route(
            "/api/withdraws/restore-all",
            post(restore_all_withdraw_handler),
        )
        .route(
            "/api/withdraws/delete-all",
            post(delete_all_withdraw_handler),
        )
        .route(
            "/api/withdraws/stats/amount/monthly",
            get(get_monthly_withdraws),
        )
        .route(
            "/api/withdraws/stats/amount/yearly",
            get(get_yearly_withdraws),
        )
        .route(
            "/api/withdraws/stats/status/success/monthly",
            get(get_month_status_success),
        )
        .route(
            "/api/withdraws/stats/status/success/yearly",
            get(get_yearly_status_success),
        )
        .route(
            "/api/withdraws/stats/status/failed/monthly",
            get(get_month_status_failed),
        )
        .route(
            "/api/withdraws/stats/status/failed/yearly",
            get(get_yearly_status_failed),
        )
        .route(
            "/api/withdraws/stats/amount/monthly/by-card",
            get(get_monthly_by_card_number),
        )
        .route(
            "/api/withdraws/stats/amount/yearly/by-card",
            get(get_yearly_by_card_number),
        )
        .route(
            "/api/withdraws/stats/status/success/monthly/by-card",
            get(get_month_status_success_by_card),
        )
        .route(
            "/api/withdraws/stats/status/success/yearly/by-card",
            get(get_yearly_status_success_by_card),
        )
        .route(
            "/api/withdraws/stats/status/failed/monthly/by-card",
            get(get_month_status_failed_by_card),
        )
        .route(
            "/api/withdraws/stats/status/failed/yearly/by-card",
            get(get_yearly_status_failed_by_card),
        )
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
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .with_state(state)
}
