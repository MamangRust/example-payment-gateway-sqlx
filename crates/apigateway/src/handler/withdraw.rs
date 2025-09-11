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
    abstract_trait::withdraw::http::{
        command::DynWithdrawCommandGrpcClient,
        query::DynWithdrawQueryGrpcClient,
        stats::{
            amount::DynWithdrawStatsAmountGrpcClient, status::DynWithdrawStatsStatusGrpcClient,
        },
        statsbycard::{
            amount::DynWithdrawStatsAmountByCardNumberGrpcClient,
            status::DynWithdrawStatsStatusByCardNumberGrpcClient,
        },
    },
    domain::{
        requests::withdraw::{
            CreateWithdrawRequest, FindAllWithdrawCardNumber, FindAllWithdraws,
            MonthStatusWithdraw, MonthStatusWithdrawCardNumber, UpdateWithdrawRequest,
            YearMonthCardNumber, YearStatusWithdrawCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawMonthlyAmountResponse, WithdrawResponse,
            WithdrawResponseDeleteAt, WithdrawResponseMonthStatusFailed,
            WithdrawResponseMonthStatusSuccess, WithdrawResponseYearStatusFailed,
            WithdrawResponseYearStatusSuccess, WithdrawYearlyAmountResponse,
        },
    },
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynWithdrawQueryGrpcClient>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawQueryGrpcClient>,
    Query(params): Query<FindAllWithdrawCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all_by_card_number(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawQueryGrpcClient>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_active(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawQueryGrpcClient>,
    Query(params): Query<FindAllWithdraws>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_trashed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/withdraws",
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
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateWithdrawRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/withdraws/{id}",
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
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateWithdrawRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.withdraw_id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
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
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trashed_withdraw(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
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
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_permanent(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Withdraw deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/withdraws/restore-all",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed withdraws restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_withdraw_handler(
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All withdraws restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/withdraws/delete-all",
    tag = "Withdraw",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed withdraws permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_withdraw_handler(
    Extension(service): Extension<DynWithdrawCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed withdraws deleted permanently"
    })))
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/monthly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly withdraw amount", body = ApiResponse<Vec<WithdrawMonthlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_withdraws(
    Extension(service): Extension<DynWithdrawStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_withdraws(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/amount/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly withdraw amount", body = ApiResponse<Vec<WithdrawYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_withdraws(
    Extension(service): Extension<DynWithdrawStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_withdraws(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusGrpcClient>,
    Query(params): Query<MonthStatusWithdraw>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_success(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/success/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly successful withdraw status", body = ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_success(
    Extension(service): Extension<DynWithdrawStatsStatusGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_success(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusGrpcClient>,
    Query(params): Query<MonthStatusWithdraw>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_failed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/withdraws/stats/status/failed/yearly",
    tag = "Withdraw Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly failed withdraw status", body = ApiResponse<Vec<WithdrawResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_status_failed(
    Extension(service): Extension<DynWithdrawStatsStatusGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_failed(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsAmountByCardNumberGrpcClient>,
    Query(params): Query<YearMonthCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_by_card_number(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsAmountByCardNumberGrpcClient>,
    Query(params): Query<YearMonthCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_by_card_number(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<MonthStatusWithdrawCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_success_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<YearStatusWithdrawCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_success_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<MonthStatusWithdrawCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_status_failed_by_card(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynWithdrawStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<YearStatusWithdrawCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_status_failed_by_card(&params).await?;
    Ok(Json(response))
}

pub fn withdraw_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/withdraws", get(get_withdraws))
        .route("/api/withdraws/by-card", get(get_withdraws_by_card_number))
        .route("/api/withdraws/:id", get(get_withdraw))
        .route("/api/withdraws/active", get(get_active_withdraws))
        .route("/api/withdraws/trashed", get(get_trashed_withdraws))
        .route("/api/withdraws", post(create_withdraw))
        .route("/api/withdraws/:id", put(update_withdraw))
        .route("/api/withdraws/trash/:id", delete(trash_withdraw_handler))
        .route("/api/withdraws/restore/:id", put(restore_withdraw_handler))
        .route("/api/withdraws/delete/:id", delete(delete_withdraw))
        .route(
            "/api/withdraws/restore-all",
            put(restore_all_withdraw_handler),
        )
        .route(
            "/api/withdraws/delete-all",
            delete(delete_all_withdraw_handler),
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
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.withdraw_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
