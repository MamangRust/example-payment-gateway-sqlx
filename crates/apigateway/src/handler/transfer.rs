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
    abstract_trait::transfer::http::DynTransferGrpcClientService,
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
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_active(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<FindAllTransfers>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_trashed(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(transfer_from): Path<String>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_transfer_from(&transfer_from).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(transfer_to): Path<String>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_transfer_to(&transfer_to).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTransferRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTransferRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.transfer_id = Some(id);
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trashed(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_permanent(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Transfer deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/transfers/restore-all",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transfers restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_transfer_handler(
    Extension(service): Extension<DynTransferGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All transfers restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/transfers/delete-all",
    tag = "Transfer",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed transfers permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_transfer_handler(
    Extension(service): Extension<DynTransferGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed transfers deleted permanently"
    })))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amounts(req.year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amounts(req.year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthStatusTransfer>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_success(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_success(req.year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthStatusTransfer>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_failed(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_failed(req.year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthYearCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amounts_sender_bycard(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthYearCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amounts_receiver_bycard(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthYearCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amounts_sender_bycard(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthYearCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amounts_receiver_bycard(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthStatusTransferCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_success_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<YearStatusTransferCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_success_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<MonthStatusTransferCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_failed_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTransferGrpcClientService>,
    Query(params): Query<YearStatusTransferCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_failed_by_card(&params).await?;
    Ok(Json(response))
}

pub fn transfer_routes(app_state: Arc<AppState>) -> OpenApiRouter {
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
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.transfer_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
