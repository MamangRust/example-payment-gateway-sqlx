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
    routing::{delete, get, put},
};
use serde_json::json;
use shared::{
    abstract_trait::merchant::http::{
        command::DynMerchantCommandGrpcClient,
        query::DynMerchantQueryGrpcClient,
        stats::{
            amount::DynMerchantStatsAmountGrpcClient, method::DynMerchantStatsMethodGrpcClient,
            totalamount::DynMerchantStatsTotalAmountGrpcClient,
        },
        statsbyapikey::{
            amount::DynMerchantStatsAmountByApiKeyGrpcClient,
            method::DynMerchantStatsMethodByApiKeyGrpcClient,
            totalamount::DynMerchantStatsTotalAmountByApiKeyGrpcClient,
        },
        statsbymerchant::{
            amount::DynMerchantStatsAmountByMerchantGrpcClient,
            method::DynMerchantStatsMethodByMerchantGrpcClient,
            totalamount::DynMerchantStatsTotalAmountByMerchantGrpcClient,
        },
        transactions::DynMerchantTransactionGrpcClient,
    },
    domain::{
        requests::merchant::{
            CreateMerchantRequest, FindAllMerchantTransactions,
            FindAllMerchantTransactionsByApiKey, FindAllMerchantTransactionsById, FindAllMerchants,
            MonthYearAmountApiKey, MonthYearAmountMerchant, MonthYearPaymentMethodApiKey,
            MonthYearPaymentMethodMerchant, MonthYearTotalAmountApiKey,
            MonthYearTotalAmountMerchant, UpdateMerchantRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
            MerchantResponseMonthlyAmount, MerchantResponseMonthlyPaymentMethod,
            MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyAmount,
            MerchantResponseYearlyPaymentMethod, MerchantResponseYearlyTotalAmount,
            MerchantTransactionResponse,
        },
    },
    errors::AppErrorHttp,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/merchants",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(FindAllMerchants),
    responses(
        (status = 200, description = "List of merchants", body = ApiResponsePagination<Vec<MerchantResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_merchants(
    Extension(service): Extension<DynMerchantQueryGrpcClient>,
    Query(params): Query<FindAllMerchants>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/active",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(FindAllMerchants),
    responses(
        (status = 200, description = "List of active merchants", body = ApiResponsePagination<Vec<MerchantResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_merchants(
    Extension(service): Extension<DynMerchantQueryGrpcClient>,
    Query(params): Query<FindAllMerchants>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_active(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/trashed",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(FindAllMerchants),
    responses(
        (status = 200, description = "List of soft-deleted merchants", body = ApiResponsePagination<Vec<MerchantResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_merchants(
    Extension(service): Extension<DynMerchantQueryGrpcClient>,
    Query(params): Query<FindAllMerchants>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_trashed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/{id}",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Merchant ID")),
    responses(
        (status = 200, description = "Merchant details", body = ApiResponse<MerchantResponse>),
        (status = 404, description = "Merchant not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_merchant(
    Extension(service): Extension<DynMerchantQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/merchants",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    request_body = CreateMerchantRequest,
    responses(
        (status = 201, description = "Merchant created", body = ApiResponse<MerchantResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_merchant(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateMerchantRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/merchants/{id}",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Merchant ID")),
    request_body = UpdateMerchantRequest,
    responses(
        (status = 200, description = "Merchant updated", body = ApiResponse<MerchantResponse>),
        (status = 404, description = "Merchant not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_merchant(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateMerchantRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.merchant_id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/merchants/trash/{id}",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Merchant ID")),
    responses(
        (status = 200, description = "Merchant soft-deleted", body = ApiResponse<MerchantResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_merchant_handler(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trash(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/api/merchants/restore/{id}",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Merchant ID")),
    responses(
        (status = 200, description = "Merchant restored", body = ApiResponse<MerchantResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_merchant_handler(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/merchants/delete/{id}",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Merchant ID")),
    responses(
        (status = 200, description = "Merchant permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_merchant(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Merchant deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/merchants/restore-all",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed merchants restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_merchant_handler(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All merchants restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/merchants/delete-all",
    tag = "Merchant",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed merchants permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_merchant_handler(
    Extension(service): Extension<DynMerchantCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed merchants deleted permanently"
    })))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/monthly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly amount", body = ApiResponse<Vec<MerchantResponseMonthlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amount(
    Extension(service): Extension<DynMerchantStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/yearly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly amount", body = ApiResponse<Vec<MerchantResponseYearlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amount(
    Extension(service): Extension<DynMerchantStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/monthly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly method", body = ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_method(
    Extension(service): Extension<DynMerchantStatsMethodGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_method(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/yearly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly method", body = ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_method(
    Extension(service): Extension<DynMerchantStatsMethodGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_method(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/monthly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly total amount", body = ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_total_amount(
    Extension(service): Extension<DynMerchantStatsTotalAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_total_amount(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/yearly",
    tag = "Merchant Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly total amount", body = ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_total_amount(
    Extension(service): Extension<DynMerchantStatsTotalAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_total_amount(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/monthly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearAmountMerchant),
    responses(
        (status = 200, description = "Monthly amount by merchant", body = ApiResponse<Vec<MerchantResponseMonthlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amount_by_merchant(
    Extension(service): Extension<DynMerchantStatsAmountByMerchantGrpcClient>,
    Query(params): Query<MonthYearAmountMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/yearly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearAmountMerchant),
    responses(
        (status = 200, description = "Yearly amount by merchant", body = ApiResponse<Vec<MerchantResponseYearlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amount_by_merchant(
    Extension(service): Extension<DynMerchantStatsAmountByMerchantGrpcClient>,
    Query(params): Query<MonthYearAmountMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/monthly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethodMerchant),
    responses(
        (status = 200, description = "Monthly method by merchant", body = ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_method_by_merchant(
    Extension(service): Extension<DynMerchantStatsMethodByMerchantGrpcClient>,
    Query(params): Query<MonthYearPaymentMethodMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_method(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/yearly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethodMerchant),
    responses(
        (status = 200, description = "Yearly method by merchant", body = ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_method_by_merchant(
    Extension(service): Extension<DynMerchantStatsMethodByMerchantGrpcClient>,
    Query(params): Query<MonthYearPaymentMethodMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_method(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/monthly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearTotalAmountMerchant),
    responses(
        (status = 200, description = "Monthly total amount by merchant", body = ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_total_amount_by_merchant(
    Extension(service): Extension<DynMerchantStatsTotalAmountByMerchantGrpcClient>,
    Query(params): Query<MonthYearTotalAmountMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_total_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/yearly/by-merchant",
    tag = "Merchant Stats By Merchant",
    security(("bearer_auth" = [])),
    params(MonthYearTotalAmountMerchant),
    responses(
        (status = 200, description = "Yearly total amount by merchant", body = ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_total_amount_by_merchant(
    Extension(service): Extension<DynMerchantStatsTotalAmountByMerchantGrpcClient>,
    Query(params): Query<MonthYearTotalAmountMerchant>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_total_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/monthly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearAmountApiKey),
    responses(
        (status = 200, description = "Monthly amount by API key", body = ApiResponse<Vec<MerchantResponseMonthlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_amount_by_apikey(
    Extension(service): Extension<DynMerchantStatsAmountByApiKeyGrpcClient>,
    Query(params): Query<MonthYearAmountApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/amount/yearly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearAmountApiKey),
    responses(
        (status = 200, description = "Yearly amount by API key", body = ApiResponse<Vec<MerchantResponseYearlyAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_amount_by_apikey(
    Extension(service): Extension<DynMerchantStatsAmountByApiKeyGrpcClient>,
    Query(params): Query<MonthYearAmountApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/monthly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethodApiKey),
    responses(
        (status = 200, description = "Monthly method by API key", body = ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_method_by_apikey(
    Extension(service): Extension<DynMerchantStatsMethodByApiKeyGrpcClient>,
    Query(params): Query<MonthYearPaymentMethodApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_method(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/method/yearly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearPaymentMethodApiKey),
    responses(
        (status = 200, description = "Yearly method by API key", body = ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_method_by_apikey(
    Extension(service): Extension<DynMerchantStatsMethodByApiKeyGrpcClient>,
    Query(params): Query<MonthYearPaymentMethodApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_method(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/monthly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearTotalAmountApiKey),
    responses(
        (status = 200, description = "Monthly total amount by API key", body = ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_total_amount_by_apikey(
    Extension(service): Extension<DynMerchantStatsTotalAmountByApiKeyGrpcClient>,
    Query(params): Query<MonthYearTotalAmountApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_total_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/stats/total-amount/yearly/by-apikey",
    tag = "Merchant Stats By API Key",
    security(("bearer_auth" = [])),
    params(MonthYearTotalAmountApiKey),
    responses(
        (status = 200, description = "Yearly total amount by API key", body = ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_total_amount_by_apikey(
    Extension(service): Extension<DynMerchantStatsTotalAmountByApiKeyGrpcClient>,
    Query(params): Query<MonthYearTotalAmountApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_total_amount(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/transactions",
    tag = "Merchant Transactions",
    security(("bearer_auth" = [])),
    params(FindAllMerchantTransactions),
    responses(
        (status = 200, description = "List of merchant transactions", body = ApiResponsePagination<Vec<MerchantTransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_merchant_transactions(
    Extension(service): Extension<DynMerchantTransactionGrpcClient>,
    Query(params): Query<FindAllMerchantTransactions>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all_transactiions(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/transactions/by-merchant",
    tag = "Merchant Transactions",
    security(("bearer_auth" = [])),
    params(FindAllMerchantTransactionsById),
    responses(
        (status = 200, description = "List of merchant transactions by merchant ID", body = ApiResponsePagination<Vec<MerchantTransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_merchant_transactions_by_id(
    Extension(service): Extension<DynMerchantTransactionGrpcClient>,
    Query(params): Query<FindAllMerchantTransactionsById>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all_transactiions_by_id(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/merchants/transactions/by-apikey",
    tag = "Merchant Transactions",
    security(("bearer_auth" = [])),
    params(FindAllMerchantTransactionsByApiKey),
    responses(
        (status = 200, description = "List of merchant transactions by API key", body = ApiResponsePagination<Vec<MerchantTransactionResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_merchant_transactions_by_apikey(
    Extension(service): Extension<DynMerchantTransactionGrpcClient>,
    Query(params): Query<FindAllMerchantTransactionsByApiKey>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all_transactiions_by_api_key(&params).await?;
    Ok(Json(response))
}

pub fn merchant_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/merchants", get(get_merchants).post(create_merchant))
        .route("/api/merchants/active", get(get_active_merchants))
        .route("/api/merchants/trashed", get(get_trashed_merchants))
        .route("/api/merchants/:id", get(get_merchant).put(update_merchant))
        .route("/api/merchants/trash/:id", delete(trash_merchant_handler))
        .route("/api/merchants/restore/:id", put(restore_merchant_handler))
        .route("/api/merchants/delete/:id", delete(delete_merchant))
        .route(
            "/api/merchants/restore-all",
            put(restore_all_merchant_handler),
        )
        .route(
            "/api/merchants/delete-all",
            delete(delete_all_merchant_handler),
        )
        .route(
            "/api/merchants/stats/amount/monthly",
            get(get_monthly_amount),
        )
        .route("/api/merchants/stats/amount/yearly", get(get_yearly_amount))
        .route(
            "/api/merchants/stats/method/monthly",
            get(get_monthly_method),
        )
        .route("/api/merchants/stats/method/yearly", get(get_yearly_method))
        .route(
            "/api/merchants/stats/total-amount/monthly",
            get(get_monthly_total_amount),
        )
        .route(
            "/api/merchants/stats/total-amount/yearly",
            get(get_yearly_total_amount),
        )
        .route(
            "/api/merchants/stats/amount/monthly/by-merchant",
            get(get_monthly_amount_by_merchant),
        )
        .route(
            "/api/merchants/stats/amount/yearly/by-merchant",
            get(get_yearly_amount_by_merchant),
        )
        .route(
            "/api/merchants/stats/method/monthly/by-merchant",
            get(get_monthly_method_by_merchant),
        )
        .route(
            "/api/merchants/stats/method/yearly/by-merchant",
            get(get_yearly_method_by_merchant),
        )
        .route(
            "/api/merchants/stats/total-amount/monthly/by-merchant",
            get(get_monthly_total_amount_by_merchant),
        )
        .route(
            "/api/merchants/stats/total-amount/yearly/by-merchant",
            get(get_yearly_total_amount_by_merchant),
        )
        .route(
            "/api/merchants/stats/amount/monthly/by-apikey",
            get(get_monthly_amount_by_apikey),
        )
        .route(
            "/api/merchants/stats/amount/yearly/by-apikey",
            get(get_yearly_amount_by_apikey),
        )
        .route(
            "/api/merchants/stats/method/monthly/by-apikey",
            get(get_monthly_method_by_apikey),
        )
        .route(
            "/api/merchants/stats/method/yearly/by-apikey",
            get(get_yearly_method_by_apikey),
        )
        .route(
            "/api/merchants/stats/total-amount/monthly/by-apikey",
            get(get_monthly_total_amount_by_apikey),
        )
        .route(
            "/api/merchants/stats/total-amount/yearly/by-apikey",
            get(get_yearly_total_amount_by_apikey),
        )
        .route(
            "/api/merchants/transactions",
            get(get_merchant_transactions),
        )
        .route(
            "/api/merchants/transactions/by-merchant",
            get(get_merchant_transactions_by_id),
        )
        .route(
            "/api/merchants/transactions/by-apikey",
            get(get_merchant_transactions_by_apikey),
        )
        .route_layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.merchant_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
