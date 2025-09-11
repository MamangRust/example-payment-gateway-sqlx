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
    abstract_trait::saldo::http::{
        command::DynSaldoCommandGrpcClient,
        query::DynSaldoQueryGrpcClient,
        stats::{balance::DynSaldoBalanceGrpcClient, total::DynSaldoTotalBalanceGrpcClient},
    },
    domain::{
        requests::saldo::{
            CreateSaldoRequest, FindAllSaldos, MonthTotalSaldoBalance, UpdateSaldoRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, SaldoMonthBalanceResponse,
            SaldoMonthTotalBalanceResponse, SaldoResponse, SaldoResponseDeleteAt,
            SaldoYearBalanceResponse, SaldoYearTotalBalanceResponse,
        },
    },
    errors::AppErrorHttp,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/saldos",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(FindAllSaldos),
    responses(
        (status = 200, description = "List of saldos", body = ApiResponsePagination<Vec<SaldoResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_saldos(
    Extension(service): Extension<DynSaldoQueryGrpcClient>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/active",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(FindAllSaldos),
    responses(
        (status = 200, description = "List of active saldos", body = ApiResponsePagination<Vec<SaldoResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_saldos(
    Extension(service): Extension<DynSaldoQueryGrpcClient>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_active(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/trashed",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(FindAllSaldos),
    responses(
        (status = 200, description = "List of soft-deleted saldos", body = ApiResponsePagination<Vec<SaldoResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_saldos(
    Extension(service): Extension<DynSaldoQueryGrpcClient>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_trashed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/{id}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Saldo ID")),
    responses(
        (status = 200, description = "Saldo details", body = ApiResponse<SaldoResponse>),
        (status = 404, description = "Saldo not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_saldo(
    Extension(service): Extension<DynSaldoQueryGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/saldos",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    request_body = CreateSaldoRequest,
    responses(
        (status = 201, description = "Saldo created", body = ApiResponse<SaldoResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_saldo(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateSaldoRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/saldos/{id}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Saldo ID")),
    request_body = UpdateSaldoRequest,
    responses(
        (status = 200, description = "Saldo updated", body = ApiResponse<SaldoResponse>),
        (status = 404, description = "Saldo not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_saldo(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateSaldoRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    // Asumsi struct UpdateSaldoRequest punya field `saldo_id`
    // Jika tidak, sesuaikan dengan struct-mu
    body.saldo_id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/saldos/trash/{id}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Saldo ID")),
    responses(
        (status = 200, description = "Saldo soft-deleted", body = ApiResponse<SaldoResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_saldo_handler(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trash(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/api/saldos/restore/{id}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Saldo ID")),
    responses(
        (status = 200, description = "Saldo restored", body = ApiResponse<SaldoResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_saldo_handler(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/saldos/delete/{id}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Saldo ID")),
    responses(
        (status = 200, description = "Saldo permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_saldo(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_permanent(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Saldo deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/saldos/restore-all",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed saldos restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_saldo_handler(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All saldos restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/saldos/delete-all",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed saldos permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_saldo_handler(
    Extension(service): Extension<DynSaldoCommandGrpcClient>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed saldos deleted permanently"
    })))
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/balance/monthly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Monthly balance", body = ApiResponse<Vec<SaldoMonthBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_balance(
    Extension(service): Extension<DynSaldoBalanceGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_balance(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/balance/yearly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly balance", body = ApiResponse<Vec<SaldoYearBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_balance(
    Extension(service): Extension<DynSaldoBalanceGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_year_balance(year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/total-balance/monthly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(MonthTotalSaldoBalance),
    responses(
        (status = 200, description = "Monthly total balance", body = ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_total_balance(
    Extension(service): Extension<DynSaldoTotalBalanceGrpcClient>,
    Query(params): Query<MonthTotalSaldoBalance>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_month_total_balance(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/total-balance/yearly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(("year" = i32, Query, description = "Tahun")),
    responses(
        (status = 200, description = "Yearly total balance", body = ApiResponse<Vec<SaldoYearTotalBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_total_balance(
    Extension(service): Extension<DynSaldoTotalBalanceGrpcClient>,
    Query(year): Query<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_year_total_balance(year).await?;
    Ok(Json(response))
}

pub fn saldo_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/saldos", get(get_saldos))
        .route("/api/saldos/active", get(get_active_saldos))
        .route("/api/saldos/trashed", get(get_trashed_saldos))
        .route("/api/saldos/:id", get(get_saldo))
        .route("/api/saldos", post(create_saldo))
        .route("/api/saldos/:id", put(update_saldo))
        .route("/api/saldos/trash/:id", delete(trash_saldo_handler))
        .route("/api/saldos/restore/:id", put(restore_saldo_handler))
        .route("/api/saldos/delete/:id", delete(delete_saldo))
        .route("/api/saldos/restore-all", put(restore_all_saldo_handler))
        .route("/api/saldos/delete-all", delete(delete_all_saldo_handler))
        .route(
            "/api/saldos/stats/balance/monthly",
            get(get_monthly_balance),
        )
        .route("/api/saldos/stats/balance/yearly", get(get_yearly_balance))
        .route(
            "/api/saldos/stats/total-balance/monthly",
            get(get_monthly_total_balance),
        )
        .route(
            "/api/saldos/stats/total-balance/yearly",
            get(get_yearly_total_balance),
        )
        .layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.saldo_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
