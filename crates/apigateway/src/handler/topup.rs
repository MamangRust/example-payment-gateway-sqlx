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
    abstract_trait::topup::http::{
        command::DynTopupCommandGrpcClient,
        query::DynTopupQueryGrpcClient,
        stats::{
            amount::DynTopupStatsAmountGrpcClient, method::DynTopupStatsMethodGrpcClient,
            status::DynTopupStatsStatusGrpcClient,
        },
        statsbycard::{
            amount::DynTopupStatsAmountByCardNumberGrpcClient,
            method::DynTopupStatsMethodByCardNumberGrpcClient,
            status::DynTopupStatsStatusByCardNumberGrpcClient,
        },
    },
    domain::{
        requests::topup::{
            CreateTopupRequest, FindAllTopups, FindAllTopupsByCardNumber, MonthTopupStatus,
            MonthTopupStatusCardNumber, UpdateTopupRequest, YearMonthMethod,
            YearTopupStatusCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TopupMonthAmountResponse, TopupMonthMethodResponse,
            TopupResponse, TopupResponseDeleteAt, TopupResponseMonthStatusFailed,
            TopupResponseMonthStatusSuccess, TopupResponseYearStatusFailed,
            TopupResponseYearStatusSuccess, TopupYearlyAmountResponse, TopupYearlyMethodResponse,
        },
    },
    errors::AppErrorHttp,
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
    Extension(service): Extension<DynTopupQueryGrpcClient>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupQueryGrpcClient>,
    Query(params): Query<FindAllTopupsByCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all_by_card_number(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupQueryGrpcClient>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_active(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupQueryGrpcClient>,
    Query(params): Query<FindAllTopups>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_trashed(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/topups",
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
    Extension(service): Extension<DynTopupCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateTopupRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/topups/{id}",
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
    Extension(service): Extension<DynTopupCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateTopupRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.topup_id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
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
    Extension(service): Extension<DynTopupCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trashed(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
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
    Extension(service): Extension<DynTopupCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_permanent(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Topup deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/topups/restore-all",
    tag = "Topup",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed topups restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_topup_handler(
    Extension(service): Extension<DynTopupCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All topups restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/topups/delete-all",
    tag = "Topup",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed topups permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_topup_handler(
    Extension(service): Extension<DynTopupCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all_permanent().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed topups deleted permanently"
    })))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly topup amount", body = ApiResponse<Vec<TopupMonthAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_amounts(
    Extension(service): Extension<DynTopupStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_amounts(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/amount/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly topup amount", body = ApiResponse<Vec<TopupYearlyAmountResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_amounts(
    Extension(service): Extension<DynTopupStatsAmountGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_amounts(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/monthly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly topup method", body = ApiResponse<Vec<TopupMonthMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_methods(
    Extension(service): Extension<DynTopupStatsMethodGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_methods(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/method/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly topup method", body = ApiResponse<Vec<TopupYearlyMethodResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_methods(
    Extension(service): Extension<DynTopupStatsMethodGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_methods(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusGrpcClient>,
    Query(params): Query<MonthTopupStatus>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_topup_status_success(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/success/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly successful topup status", body = ApiResponse<Vec<TopupResponseYearStatusSuccess>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_success(
    Extension(service): Extension<DynTopupStatsStatusGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_status_success(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusGrpcClient>,
    Query(params): Query<MonthTopupStatus>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_topup_status_failed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/topups/stats/status/failed/yearly",
    tag = "Topup Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly failed topup status", body = ApiResponse<Vec<TopupResponseYearStatusFailed>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_status_failed(
    Extension(service): Extension<DynTopupStatsStatusGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_status_failed(year).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsAmountByCardNumberGrpcClient>,
    Query(params): Query<YearMonthMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_amounts(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsAmountByCardNumberGrpcClient>,
    Query(params): Query<YearMonthMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_amounts(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsMethodByCardNumberGrpcClient>,
    Query(params): Query<YearMonthMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_methods(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsMethodByCardNumberGrpcClient>,
    Query(params): Query<YearMonthMethod>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_methods(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<MonthTopupStatusCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_topup_status_success(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<YearTopupStatusCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_status_success(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<MonthTopupStatusCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_topup_status_failed(&params).await?;
    Ok(Json(response))
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
    Extension(service): Extension<DynTopupStatsStatusByCardNumberGrpcClient>,
    Query(params): Query<YearTopupStatusCardNumber>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_status_failed(&params).await?;
    Ok(Json(response))
}

pub fn topup_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/topups", get(get_topups))
        .route("/api/topups/by-card", get(get_topups_by_card_number))
        .route("/api/topups/active", get(get_active_topups))
        .route("/api/topups/trashed", get(get_trashed_topups))
        .route("/api/topups/{id}", get(get_topup))
        .route("/api/topups", post(create_topup))
        .route("/api/topups/{id}", put(update_topup))
        .route("/api/topups/trash/{id}", delete(trash_topup_handler))
        .route("/api/topups/restore/{id}", put(restore_topup_handler))
        .route("/api/topups/delete/{id}", delete(delete_topup))
        .route("/api/topups/restore-all", put(restore_all_topup_handler))
        .route("/api/topups/delete-all", delete(delete_all_topup_handler))
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
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.topup_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
