use crate::{
    middleware::{
        api_key::ApiKey, circuit_breaker::circuit_breaker_middleware, jwt,
        rate_limit::rate_limit_middleware, request_limiter::request_limiter_middleware,
        session::session_middleware, validate::SimpleValidatedJson,
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
        requests::{
            transaction::{
                CreateTransactionRequest, FindAllTransactionCardNumber, FindAllTransactions,
                MonthStatusTransaction, MonthStatusTransactionCardNumber, MonthYearPaymentMethod,
                UpdateTransactionRequest, YearStatusTransactionCardNumber,
            },
            withdraw::YearQuery,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransactionMonthAmountResponse,
            TransactionMonthMethodResponse, TransactionResponse, TransactionResponseDeleteAt,
            TransactionResponseMonthStatusFailed, TransactionResponseMonthStatusSuccess,
            TransactionResponseYearStatusFailed, TransactionResponseYearStatusSuccess,
            TransactionYearMethodResponse, TransactionYearlyAmountResponse,
        },
    },
    errors::HttpError,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/transactions",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(FindAllTransactions),
    responses(
        (status = 200, description = "List of transactions", body = ApiResponsePagination<Vec<TransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/by-card",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(FindAllTransactionCardNumber),
    responses(
        (status = 200, description = "List of transactions by card number", body = ApiResponsePagination<Vec<TransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions_by_card_number(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransactionCardNumber>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_all_by_card_number(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/active",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(FindAllTransactions),
    responses(
        (status = 200, description = "List of active transactions", body = ApiResponsePagination<Vec<TransactionResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_transactions(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_by_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/trashed",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(FindAllTransactions),
    responses(
        (status = 200, description = "List of soft-deleted transactions", body = ApiResponsePagination<Vec<TransactionResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_transactions(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_by_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/{id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transaction ID")),
    responses(
        (status = 200, description = "Transaction details", body = ApiResponse<TransactionResponse>),
        (status = 404, description = "Transaction not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_transaction(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/merchant/{merchant_id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("merchant_id" = i32, Path, description = "Merchant ID")),
    responses(
        (status = 200, description = "List of transactions by merchant", body = ApiResponse<Vec<TransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions_by_merchant_id(
    State(app_state): State<Arc<AppState>>,
    Path(merchant_id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.find_by_merchant_id(merchant_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/create",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    request_body = CreateTransactionRequest,
    responses(
        (status = 201, description = "Transaction created", body = ApiResponse<TransactionResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_transaction(
    ApiKey(key): ApiKey,
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTransactionRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    match transaction_client.create(&key, &body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/update/{id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transaction ID")),
    request_body = UpdateTransactionRequest,
    responses(
        (status = 200, description = "Transaction updated", body = ApiResponse<TransactionResponse>),
        (status = 404, description = "Transaction not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_transaction(
    ApiKey(key): ApiKey,
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTransactionRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

    body.transaction_id = Some(id);
    match transaction_client.update(&key, &body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/trash/{id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transaction ID")),
    responses(
        (status = 200, description = "Transaction soft-deleted", body = ApiResponse<TransactionResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_transaction_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.trashed(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/restore/{id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transaction ID")),
    responses(
        (status = 200, description = "Transaction restored", body = ApiResponse<TransactionResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_transaction_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    delete,
    path = "/api/transactions/delete/{id}",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Transaction ID")),
    responses(
        (status = 200, description = "Transaction permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_transaction(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Transaction deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/restore-all",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transactions restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_transaction_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All transactions restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/transactions/delete-all",
    tag = "Transaction",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transactions permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_transaction_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed transactions deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/amount/monthly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transaction amount", body = ApiResponse<Vec<TransactionMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amounts(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_monthly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/amount/yearly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transaction amount", body = ApiResponse<Vec<TransactionYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amounts(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_amounts(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/method/monthly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transaction method", body = ApiResponse<Vec<TransactionMonthMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_method(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_monthly_method(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/method/yearly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transaction method", body = ApiResponse<Vec<TransactionYearMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_method(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_method(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/success/monthly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusTransaction),
    responses(
        (status = 200, description = "Monthly successful transaction status", body = ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransaction>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_month_status_success(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/success/yearly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly successful transaction status", body = ApiResponse<Vec<TransactionResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_status_success(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/failed/monthly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(MonthStatusTransaction),
    responses(
        (status = 200, description = "Monthly failed transaction status", body = ApiResponse<Vec<TransactionResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransaction>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_month_status_failed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/failed/yearly",
    tag = "Transaction Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly failed transaction status", body = ApiResponse<Vec<TransactionResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_status_failed(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/amount/monthly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethod),
    responses(
        (status = 200, description = "Monthly transaction amount by card", body = ApiResponse<Vec<TransactionMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amounts_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearPaymentMethod>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_monthly_amounts_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/amount/yearly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethod),
    responses(
        (status = 200, description = "Yearly transaction amount by card", body = ApiResponse<Vec<TransactionYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amounts_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearPaymentMethod>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_amounts_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/method/monthly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethod),
    responses(
        (status = 200, description = "Monthly transaction method by card", body = ApiResponse<Vec<TransactionMonthMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_method_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearPaymentMethod>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_monthly_method_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/method/yearly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethod),
    responses(
        (status = 200, description = "Yearly transaction method by card", body = ApiResponse<Vec<TransactionYearMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_method_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthYearPaymentMethod>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client.get_yearly_method_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/success/monthly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusTransactionCardNumber),
    responses(
        (status = 200, description = "Monthly successful transaction status by card", body = ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransactionCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client
        .get_month_status_success_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/success/yearly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusTransactionCardNumber),
    responses(
        (status = 200, description = "Yearly successful transaction status by card", body = ApiResponse<Vec<TransactionResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusTransactionCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client
        .get_yearly_status_success_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/failed/monthly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthStatusTransactionCardNumber),
    responses(
        (status = 200, description = "Monthly failed transaction status by card", body = ApiResponse<Vec<TransactionResponseMonthStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_month_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthStatusTransactionCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client
        .get_month_status_failed_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/transactions/stats/status/failed/yearly/by-card",
    tag = "Transaction Stats By Card",
    security(("bearer_auth" = [])),
    params(YearStatusTransactionCardNumber),
    responses(
        (status = 200, description = "Yearly failed transaction status by card", body = ApiResponse<Vec<TransactionResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed_by_card(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<YearStatusTransactionCardNumber>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let transaction_client = &app_state.di_container.transaction_clients;

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

    match transaction_client
        .get_yearly_status_failed_bycard(&params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

pub fn transaction_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/transactions", get(get_transactions))
        .route(
            "/api/transactions/by-card",
            get(get_transactions_by_card_number),
        )
        .route("/api/transactions/active", get(get_active_transactions))
        .route("/api/transactions/trashed", get(get_trashed_transactions))
        .route("/api/transactions/{id}", get(get_transaction))
        .route(
            "/api/transactions/merchant/{merchant_id}",
            get(get_transactions_by_merchant_id),
        )
        .route("/api/transactions/create", post(create_transaction))
        .route("/api/transactions/update/{id}", post(update_transaction))
        .route(
            "/api/transactions/trash/{id}",
            post(trash_transaction_handler),
        )
        .route(
            "/api/transactions/restore/{id}",
            post(restore_transaction_handler),
        )
        .route("/api/transactions/delete/{id}", delete(delete_transaction))
        .route(
            "/api/transactions/restore-all",
            post(restore_all_transaction_handler),
        )
        .route(
            "/api/transactions/delete-all",
            post(delete_all_transaction_handler),
        )
        .route(
            "/api/transactions/stats/amount/monthly",
            get(get_monthly_amounts),
        )
        .route(
            "/api/transactions/stats/amount/yearly",
            get(get_yearly_amounts),
        )
        .route(
            "/api/transactions/stats/method/monthly",
            get(get_monthly_method),
        )
        .route(
            "/api/transactions/stats/method/yearly",
            get(get_yearly_method),
        )
        .route(
            "/api/transactions/stats/status/success/monthly",
            get(get_month_status_success),
        )
        .route(
            "/api/transactions/stats/status/success/yearly",
            get(get_yearly_status_success),
        )
        .route(
            "/api/transactions/stats/status/failed/monthly",
            get(get_month_status_failed),
        )
        .route(
            "/api/transactions/stats/status/failed/yearly",
            get(get_yearly_status_failed),
        )
        .route(
            "/api/transactions/stats/amount/monthly/by-card",
            get(get_monthly_amounts_by_card),
        )
        .route(
            "/api/transactions/stats/amount/yearly/by-card",
            get(get_yearly_amounts_by_card),
        )
        .route(
            "/api/transactions/stats/method/monthly/by-card",
            get(get_monthly_method_by_card),
        )
        .route(
            "/api/transactions/stats/method/yearly/by-card",
            get(get_yearly_method_by_card),
        )
        .route(
            "/api/transactions/stats/status/success/monthly/by-card",
            get(get_month_status_success_by_card),
        )
        .route(
            "/api/transactions/stats/status/success/yearly/by-card",
            get(get_yearly_status_success_by_card),
        )
        .route(
            "/api/transactions/stats/status/failed/monthly/by-card",
            get(get_month_status_failed_by_card),
        )
        .route(
            "/api/transactions/stats/status/failed/yearly/by-card",
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
