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
        requests::{
            saldo::{
                CreateSaldoRequest, FindAllSaldos, MonthTotalSaldoBalance, UpdateSaldoRequest,
            },
            withdraw::YearQuery,
        },
        responses::{
            ApiResponse, ApiResponsePagination, SaldoMonthBalanceResponse,
            SaldoMonthTotalBalanceResponse, SaldoResponse, SaldoResponseDeleteAt,
            SaldoYearBalanceResponse, SaldoYearTotalBalanceResponse,
        },
    },
    errors::HttpError,
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
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.find_all(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
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
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.find_active(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
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
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<FindAllSaldos>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.find_trashed(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
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
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.find_by_id(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/saldos/by-card/{card_number}",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    params(("card_number" = String, Path, description = "Card Number")),
    responses(
        (status = 200, description = "Saldo details", body = ApiResponse<SaldoResponse>),
        (status = 404, description = "Saldo not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_saldo_by_card(
    State(app_state): State<Arc<AppState>>,
    Path(card_number): Path<String>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.find_by_card(&card_number).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/saldos/create",
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
    State(app_state): State<Arc<AppState>>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateSaldoRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    match saldo_client.create(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/saldos/update/{id}",
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
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateSaldoRequest>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

    body.saldo_id = Some(id);
    match saldo_client.update(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
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
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.trash(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
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
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.restore(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
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
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.delete_permanent(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Saldo deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/saldos/restore-all",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed saldos restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_saldo_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.restore_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All saldos restored successfully"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/saldos/delete-all",
    tag = "Saldo",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed saldos permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_saldo_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.delete_all().await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
               "status": "success",
               "message": "All trashed saldos deleted permanently"
            })),
        )),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/balance/monthly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly balance", body = ApiResponse<Vec<SaldoMonthBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_balance(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.get_month_balance(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/balance/yearly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly balance", body = ApiResponse<Vec<SaldoYearBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_balance(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.get_year_balance(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
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
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<MonthTotalSaldoBalance>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.get_month_total_balance(&params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/saldos/stats/total-balance/yearly",
    tag = "Saldo Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly total balance", body = ApiResponse<Vec<SaldoYearTotalBalanceResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_total_balance(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<YearQuery>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, HttpError> {
    let saldo_client = &app_state.di_container.saldo_clients;

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

    match saldo_client.get_year_total_balance(req.year).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err),
    }
}

pub fn saldo_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/saldos", get(get_saldos))
        .route("/api/saldos/create", post(create_saldo))
        .route("/api/saldos/update/{id}", post(update_saldo))
        .route("/api/saldos/active", get(get_active_saldos))
        .route("/api/saldos/trashed", get(get_trashed_saldos))
        .route("/api/saldos/{id}", get(get_saldo))
        .route("/api/saldos/by-card/{card_number}", get(get_saldo_by_card))
        .route("/api/saldos/trash/{id}", post(trash_saldo_handler))
        .route("/api/saldos/restore/{id}", post(restore_saldo_handler))
        .route("/api/saldos/delete/{id}", delete(delete_saldo))
        .route("/api/saldos/restore-all", post(restore_all_saldo_handler))
        .route("/api/saldos/delete-all", post(delete_all_saldo_handler))
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
