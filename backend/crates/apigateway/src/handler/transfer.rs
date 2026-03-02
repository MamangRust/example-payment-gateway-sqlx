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
    routing::{delete, get, post, put},
};
use serde_json::json;
use shared::{
    domain::{
        requests::{
            transfer::{
                CreateTransferRequest, FindAllTransfers, MonthStatusTransfer,
                MonthStatusTransferCardNumber, MonthYearCardNumber, UpdateTransferRequest,
                YearStatusTransferCardNumber,
            },
            withdraw::YearQuery,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransferMonthAmountResponse, TransferResponse,
            TransferResponseDeleteAt, TransferResponseMonthStatusFailed,
            TransferResponseMonthStatusSuccess, TransferResponseYearStatusFailed,
            TransferResponseYearStatusSuccess, TransferYearAmountResponse,
        },
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/transfers",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(FindAllTransfers),
    responses(
        (status = 200, description = "List of transfers", body = ApiResponsePagination<Vec<TransferResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transfers(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/{id}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transfer ID")),
    responses(
        (status = 200, description = "Transfer details", body = ApiResponse<TransferResponse>),
        (status = 404, description = "Transfer not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_transfer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/active",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(FindAllTransfers),
    responses(
        (status = 200, description = "List of active transfers", body = ApiResponsePagination<Vec<TransferResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_transfers(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_by_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/trashed",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(FindAllTransfers),
    responses(
        (status = 200, description = "List of soft-deleted transfers", body = ApiResponsePagination<Vec<TransferResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_transfers(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_by_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/from/{transfer_from}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("transfer_from" = String, Path, description = "Transfer from card number")),
    responses(
        (status = 200, description = "List of transfers by sender", body = ApiResponse<Vec<TransferResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transfers_by_transfer_from(
    State(app_state): State<Arc<AppState>>,
    Path(transfer_from): Path<String>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_by_transfer_from(&transfer_from).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/to/{transfer_to}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("transfer_to" = String, Path, description = "Transfer to card number")),
    responses(
        (status = 200, description = "List of transfers by receiver", body = ApiResponse<Vec<TransferResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transfers_by_transfer_to(
    State(app_state): State<Arc<AppState>>,
    Path(transfer_to): Path<String>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.find_by_transfer_to(&transfer_to).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/create",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    request_body = CreateTransferRequest,
    responses(
        (status = 201, description = "Transfer created", body = ApiResponse<TransferResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_transfer(
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTransferRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    match transfer_client.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/update/{id}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transfer ID")),
    request_body = UpdateTransferRequest,
    responses(
        (status = 200, description = "Transfer updated", body = ApiResponse<TransferResponse>),
        (status = 404, description = "Transfer not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_transfer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTransferRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

    body.transfer_id = Some(id);
    match transfer_client.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/trash/{id}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transfer ID")),
    responses(
        (status = 200, description = "Transfer soft-deleted", body = ApiResponse<TransferResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_transfer_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.trashed(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/restore/{id}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transfer ID")),
    responses(
        (status = 200, description = "Transfer restored", body = ApiResponse<TransferResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_transfer_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/transfers/delete/{id}",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transfer ID")),
    responses(
        (status = 200, description = "Transfer permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_transfer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Transfer deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/restore-all",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transfers restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_transfer_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All transfers restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers/delete-all",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transfers permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_transfer_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed transfers deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/monthly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transfer amount", body = ApiResponse<Vec<TransferMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amounts(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_monthly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/yearly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transfer amount", body = ApiResponse<Vec<TransferYearAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amounts(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_yearly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/success/monthly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusTransfer),
    responses(
        (status = 200, description = "Monthly successful transfer status", body = ApiResponse<Vec<TransferResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransfer>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_month_status_success(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/success/yearly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly successful transfer status", body = ApiResponse<Vec<TransferResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_yearly_status_success(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/failed/monthly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusTransfer),
    responses(
        (status = 200, description = "Monthly failed transfer status", body = ApiResponse<Vec<TransferResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransfer>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_month_status_failed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/failed/yearly",
    tag = "Transfer Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly failed transfer status", body = ApiResponse<Vec<TransferResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client.get_yearly_status_failed(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/monthly/sender",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumber),
    responses(
        (status = 200, description = "Monthly transfer amount by sender", body = ApiResponse<Vec<TransferMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amounts_by_sender(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_monthly_amounts_sender_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/monthly/receiver",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumber),
    responses(
        (status = 200, description = "Monthly transfer amount by receiver", body = ApiResponse<Vec<TransferMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amounts_by_receiver(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_monthly_amounts_receiver_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/yearly/sender",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumber),
    responses(
        (status = 200, description = "Yearly transfer amount by sender", body = ApiResponse<Vec<TransferYearAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amounts_by_sender(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_yearly_amounts_sender_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/amount/yearly/receiver",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumber),
    responses(
        (status = 200, description = "Yearly transfer amount by receiver", body = ApiResponse<Vec<TransferYearAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amounts_by_receiver(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_yearly_amounts_receiver_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/success/monthly/by-card",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusTransferCardNumber),
    responses(
        (status = 200, description = "Monthly successful transfer status by card", body = ApiResponse<Vec<TransferResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransferCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_month_status_success_by_card(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/success/yearly/by-card",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusTransferCardNumber),
    responses(
        (status = 200, description = "Yearly successful transfer status by card", body = ApiResponse<Vec<TransferResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusTransferCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_yearly_status_success_by_card(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/failed/monthly/by-card",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusTransferCardNumber),
    responses(
        (status = 200, description = "Monthly failed transfer status by card", body = ApiResponse<Vec<TransferResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransferCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_month_status_failed_by_card(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/stats/status/failed/yearly/by-card",
    tag = "Transfer Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusTransferCardNumber),
    responses(
        (status = 200, description = "Yearly failed transfer status by card", body = ApiResponse<Vec<TransferResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusTransferCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transfer_client = &app_state.di_container.transfer_clients;

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

    match transfer_client
        .get_yearly_status_failed_by_card(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}
pub fn transfer_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/transfers", get(get_transfers))
        .route("/api/transfers/{id}", get(get_transfer))
        .route("/api/transfers/active", get(get_active_transfers))
        .route("/api/transfers/trashed", get(get_trashed_transfers))
        .route(
            "/api/transfers/from/{transfer_from}",
            get(get_transfers_by_transfer_from),
        )
        .route(
            "/api/transfers/to/{transfer_to}",
            get(get_transfers_by_transfer_to),
        )
        .route("/api/transfers/create", post(create_transfer))
        .route("/api/transfers/update/{id}", post(update_transfer))
        .route("/api/transfers/trash/{id}", delete(trash_transfer_handler))
        .route(
            "/api/transfers/restore/{id}",
            post(restore_transfer_handler),
        )
        .route("/api/transfers/delete/{id}", post(delete_transfer))
        .route(
            "/api/transfers/restore-all",
            put(restore_all_transfer_handler),
        )
        .route(
            "/api/transfers/delete-all",
            delete(delete_all_transfer_handler),
        )
        .route(
            "/api/transfers/stats/amount/monthly",
            get(get_monthly_amounts),
        )
        .route(
            "/api/transfers/stats/amount/yearly",
            get(get_yearly_amounts),
        )
        .route(
            "/api/transfers/stats/status/success/monthly",
            get(get_month_status_success),
        )
        .route(
            "/api/transfers/stats/status/success/yearly",
            get(get_yearly_status_success),
        )
        .route(
            "/api/transfers/stats/status/failed/monthly",
            get(get_month_status_failed),
        )
        .route(
            "/api/transfers/stats/status/failed/yearly",
            get(get_yearly_status_failed),
        )
        .route(
            "/api/transfers/stats/amount/monthly/sender",
            get(get_monthly_amounts_by_sender),
        )
        .route(
            "/api/transfers/stats/amount/monthly/receiver",
            get(get_monthly_amounts_by_receiver),
        )
        .route(
            "/api/transfers/stats/amount/yearly/sender",
            get(get_yearly_amounts_by_sender),
        )
        .route(
            "/api/transfers/stats/amount/yearly/receiver",
            get(get_yearly_amounts_by_receiver),
        )
        .route(
            "/api/transfers/stats/status/success/monthly/by-card",
            get(get_month_status_success_by_card),
        )
        .route(
            "/api/transfers/stats/status/success/yearly/by-card",
            get(get_yearly_status_success_by_card),
        )
        .route(
            "/api/transfers/stats/status/failed/monthly/by-card",
            get(get_month_status_failed_by_card),
        )
        .route(
            "/api/transfers/stats/status/failed/yearly/by-card",
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
