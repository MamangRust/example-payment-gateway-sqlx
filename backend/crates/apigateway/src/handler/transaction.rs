use crate::{
    middleware::{
        api_key::ApiKey, jwt, rate_limit::rate_limit_middleware, validate::SimpleValidatedJson,
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
    abstract_trait::transaction::http::DynTransactionGrpcClientService,
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
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_all(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<FindAllTransactionCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_all_by_card_number(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_by_active(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<FindAllTransactions>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_by_trashed(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_by_id(id).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(merchant_id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.find_by_merchant_id(merchant_id).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTransactionRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.create(&key, &body).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTransactionRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.transaction_id = Some(id);
    match service.update(&key, &body).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.trashed(id).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.restore(id).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.delete_permanent(id).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.restore_all().await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.delete_all().await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_monthly_amounts(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_amounts(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_monthly_method(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_method(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthStatusTransaction>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_month_status_success(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_status_success(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthStatusTransaction>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_month_status_failed(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_status_failed(req.year).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthYearPaymentMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_monthly_amounts_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthYearPaymentMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_amounts_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthYearPaymentMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_monthly_method_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthYearPaymentMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_method_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthStatusTransactionCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_month_status_success_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<YearStatusTransactionCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_status_success_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<MonthStatusTransactionCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_month_status_failed_bycard(&params).await {
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
    Extension(service): Extension<DynTransactionGrpcClientService>,
    Query(params): Query<YearStatusTransactionCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    match service.get_yearly_status_failed_bycard(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

pub fn transaction_routes(app_state: Arc<AppState>) -> OpenApiRouter {
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
        .route_layer(middleware::from_fn(rate_limit_middleware))
        .route_layer(middleware::from_fn(jwt::auth))
        .layer(Extension(
            app_state.di_container.transaction_clients.clone(),
        ))
        .layer(Extension(app_state.rate_limit.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
