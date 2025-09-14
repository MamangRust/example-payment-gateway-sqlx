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
    routing::{delete, get, post},
};
use serde_json::json;
use shared::{
    abstract_trait::card::http::DynCardGrpcClientService,
    domain::{
        requests::{
            card::{CreateCardRequest, FindAllCards, MonthYearCardNumberCard, UpdateCardRequest},
            withdraw::YearQuery,
        },
        responses::{
            ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt,
            CardResponseMonthAmount, CardResponseMonthBalance, CardResponseYearAmount,
            CardResponseYearlyBalance, DashboardCard, DashboardCardCardNumber,
        },
    },
    errors::AppErrorHttp,
};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/cards",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(FindAllCards),
    responses(
        (status = 200, description = "List of cards", body = ApiResponsePagination<Vec<CardResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_cards(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<FindAllCards>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_all(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/active",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(FindAllCards),
    responses(
        (status = 200, description = "List of active cards", body = ApiResponsePagination<Vec<CardResponse>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_active_cards(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<FindAllCards>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_active(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/trashed",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(FindAllCards),
    responses(
        (status = 200, description = "List of soft-deleted cards", body = ApiResponsePagination<Vec<CardResponseDeleteAt>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_trashed_cards(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<FindAllCards>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_trashed(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/{id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Card ID")),
    responses(
        (status = 200, description = "Card details", body = ApiResponse<CardResponse>),
        (status = 404, description = "Card not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_id(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/by-user/{user_id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("user_id" = i32, Path, description = "User ID")),
    responses(
        (status = 200, description = "Card by user", body = ApiResponse<CardResponse>),
        (status = 404, description = "Card not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_card_by_user(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_user_id(user_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/by-card/{card_number}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("card_number" = String, Path, description = "Card Number")),
    responses(
        (status = 200, description = "Card by number", body = ApiResponse<CardResponse>),
        (status = 404, description = "Card not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_card_by_number(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(card_number): Path<String>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.find_by_card_number(card_number).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/cards/create",
    tag = "Card",
    security(("bearer_auth" = [])),
    request_body = CreateCardRequest,
    responses(
        (status = 201, description = "Card created", body = ApiResponse<CardResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    SimpleValidatedJson(body): SimpleValidatedJson<CreateCardRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.create(&body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/cards/update/{id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Card ID")),
    request_body = UpdateCardRequest,
    responses(
        (status = 200, description = "Card updated", body = ApiResponse<CardResponse>),
        (status = 404, description = "Card not found"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(id): Path<i32>,
    SimpleValidatedJson(mut body): SimpleValidatedJson<UpdateCardRequest>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    body.card_id = id;
    let response = service.update(&body).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/cards/trash/{id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Card ID")),
    responses(
        (status = 200, description = "Card soft-deleted", body = ApiResponse<CardResponseDeleteAt>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn trash_card_handler(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.trash(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    put,
    path = "/api/cards/restore/{id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Card ID")),
    responses(
        (status = 200, description = "Card restored", body = ApiResponse<CardResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_card_handler(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.restore(id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/cards/delete/{id}",
    tag = "Card",
    security(("bearer_auth" = [])),
    params(("id" = i32, Path, description = "Card ID")),
    responses(
        (status = 200, description = "Card permanently deleted", body = serde_json::Value),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete(id).await?;
    Ok(Json(json!({
        "status": "success",
        "message": "Card deleted permanently"
    })))
}

#[utoipa::path(
    put,
    path = "/api/cards/restore-all",
    tag = "Card",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed cards restored", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn restore_all_card_handler(
    Extension(service): Extension<DynCardGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.restore_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All cards restored successfully"
    })))
}

#[utoipa::path(
    delete,
    path = "/api/cards/delete-all",
    tag = "Card",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "All trashed cards permanently deleted", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_all_card_handler(
    Extension(service): Extension<DynCardGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    service.delete_all().await?;
    Ok(Json(json!({
        "status": "success",
        "message": "All trashed cards deleted permanently"
    })))
}

// Balance

#[utoipa::path(
    get,
    path = "/api/cards/stats/balance/monthly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly balance", body = ApiResponse<Vec<CardResponseMonthBalance>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_balance(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_balance(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/balance/yearly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly balance", body = ApiResponse<Vec<CardResponseYearlyBalance>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_balance(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_balance(req.year).await?;
    Ok(Json(response))
}

// Topup

#[utoipa::path(
    get,
    path = "/api/cards/stats/topup/monthly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly topup amount", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_amount(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/topup/yearly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly topup amount", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_amount(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transaction/monthly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transaction amount", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transaction_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_transaction_amount(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transaction/yearly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transaction amount", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transaction_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_transaction_amount(req.year).await?;
    Ok(Json(response))
}

// Transfer

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/monthly/sender",
    tag = "Card Stats (Sender)",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transfer amount (sender)", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transfer_amount_sender(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount_sender(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/monthly/receiver",
    tag = "Card Stats (Receiver)",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly transfer amount (receiver)", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transfer_amount_receiver(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount_receiver(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/yearly/sender",
    tag = "Card Stats (Sender)",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transfer amount (sender)", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transfer_amount_sender(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount_sender(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/yearly/receiver",
    tag = "Card Stats (Receiver)",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly transfer amount (receiver)", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transfer_amount_receiver(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount_receiver(req.year).await?;
    Ok(Json(response))
}

// Withdraw

#[utoipa::path(
    get,
    path = "/api/cards/stats/withdraw/monthly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Monthly withdraw amount", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_withdraw_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_withdraw_amount(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/withdraw/yearly",
    tag = "Card Stats",
    security(("bearer_auth" = [])),
    params(YearQuery),
    responses(
        (status = 200, description = "Yearly withdraw amount", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_withdraw_amount(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(req): Query<YearQuery>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_withdraw_amount(req.year).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/balance/monthly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly balance by card", body = ApiResponse<Vec<CardResponseMonthBalance>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]

// Stats by card

pub async fn get_monthly_balance_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_balance_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/balance/yearly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly balance by card", body = ApiResponse<Vec<CardResponseYearlyBalance>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_balance_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_balance_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/topup/monthly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly topup amount by card", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_topup_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_topup_amount_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/topup/yearly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly topup amount by card", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_topup_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_topup_amount_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transaction/monthly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly transaction amount by card", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transaction_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service
        .get_monthly_transaction_amount_bycard(&params)
        .await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transaction/yearly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly transaction amount by card", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transaction_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service
        .get_yearly_transaction_amount_bycard(&params)
        .await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/monthly/by-card/sender",
    tag = "Card Stats By Card (Sender)",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly transfer amount by card (sender)", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transfer_amount_by_card_sender(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount_sender_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/monthly/by-card/receiver",
    tag = "Card Stats By Card (Receiver)",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly transfer amount by card (receiver)", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_transfer_amount_by_card_receiver(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_amount_receiver_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/yearly/by-card/sender",
    tag = "Card Stats By Card (Sender)",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly transfer amount by card (sender)", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transfer_amount_by_card_sender(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount_sender_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/transfer/yearly/by-card/receiver",
    tag = "Card Stats By Card (Receiver)",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly transfer amount by card (receiver)", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_transfer_amount_by_card_receiver(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_amount_receiver_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/withdraw/monthly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Monthly withdraw amount by card", body = ApiResponse<Vec<CardResponseMonthAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_monthly_withdraw_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_monthly_withdraw_amount_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/stats/withdraw/yearly/by-card",
    tag = "Card Stats By Card",
    security(("bearer_auth" = [])),
    params(MonthYearCardNumberCard),
    responses(
        (status = 200, description = "Yearly withdraw amount by card", body = ApiResponse<Vec<CardResponseYearAmount>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_yearly_withdraw_amount_by_card(
    Extension(service): Extension<DynCardGrpcClientService>,
    Query(params): Query<MonthYearCardNumberCard>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_yearly_withdraw_amount_bycard(&params).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/dashboard",
    tag = "Card Dashboard",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Card dashboard summary", body = ApiResponse<DashboardCard>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_card_dashboard(
    Extension(service): Extension<DynCardGrpcClientService>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_dashboard().await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/cards/dashboard/{card_number}",
    tag = "Card Dashboard",
    security(("bearer_auth" = [])),
    params(("card_number" = String, Path, description = "Card Number")),
    responses(
        (status = 200, description = "Card dashboard by card number", body = ApiResponse<DashboardCardCardNumber>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_card_dashboard_by_card_number(
    Extension(service): Extension<DynCardGrpcClientService>,
    Path(card_number): Path<String>,
) -> Result<impl IntoResponse, AppErrorHttp> {
    let response = service.get_dashboard_bycard(&card_number).await?;
    Ok(Json(response))
}

pub fn card_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/cards", get(get_cards))
        .route("/api/cards/create", post(create_card))
        .route("/api/cards/update", post(update_card))
        .route("/api/cards/active", get(get_active_cards))
        .route("/api/cards/trashed", get(get_trashed_cards))
        .route("/api/cards/{id}", get(get_card))
        .route("/api/cards/by-user/{user_id}", get(get_card_by_user))
        .route(
            "/api/cards/by-number/{card_number}",
            get(get_card_by_number),
        )
        .route("/api/cards/trash/{id}", post(trash_card_handler))
        .route("/api/cards/restore/{id}", post(restore_card_handler))
        .route("/api/cards/delete/{id}", delete(delete_card))
        .route("/api/cards/restore-all", post(restore_all_card_handler))
        .route("/api/cards/delete-all", post(delete_all_card_handler))
        .route("/api/cards/stats/balance/monthly", get(get_monthly_balance))
        .route("/api/cards/stats/balance/yearly", get(get_yearly_balance))
        .route(
            "/api/cards/stats/topup/monthly",
            get(get_monthly_topup_amount),
        )
        .route(
            "/api/cards/stats/topup/yearly",
            get(get_yearly_topup_amount),
        )
        .route(
            "/api/cards/stats/transaction/monthly",
            get(get_monthly_transaction_amount),
        )
        .route(
            "/api/cards/stats/transaction/yearly",
            get(get_yearly_transaction_amount),
        )
        .route(
            "/api/cards/stats/transfer/monthly/sender",
            get(get_monthly_transfer_amount_sender),
        )
        .route(
            "/api/cards/stats/transfer/monthly/receiver",
            get(get_monthly_transfer_amount_receiver),
        )
        .route(
            "/api/cards/stats/transfer/yearly/sender",
            get(get_yearly_transfer_amount_sender),
        )
        .route(
            "/api/cards/stats/transfer/yearly/receiver",
            get(get_yearly_transfer_amount_receiver),
        )
        .route(
            "/api/cards/stats/withdraw/monthly",
            get(get_monthly_withdraw_amount),
        )
        .route(
            "/api/cards/stats/withdraw/yearly",
            get(get_yearly_withdraw_amount),
        )
        .route(
            "/api/cards/stats/balance/monthly/by-card",
            get(get_monthly_balance_by_card),
        )
        .route(
            "/api/cards/stats/balance/yearly/by-card",
            get(get_yearly_balance_by_card),
        )
        .route(
            "/api/cards/stats/topup/monthly/by-card",
            get(get_monthly_topup_amount_by_card),
        )
        .route(
            "/api/cards/stats/topup/yearly/by-card",
            get(get_yearly_topup_amount_by_card),
        )
        .route(
            "/api/cards/stats/transaction/monthly/by-card",
            get(get_monthly_transaction_amount_by_card),
        )
        .route(
            "/api/cards/stats/transaction/yearly/by-card",
            get(get_yearly_transaction_amount_by_card),
        )
        .route(
            "/api/cards/stats/transfer/monthly/by-card/sender",
            get(get_monthly_transfer_amount_by_card_sender),
        )
        .route(
            "/api/cards/stats/transfer/monthly/by-card/receiver",
            get(get_monthly_transfer_amount_by_card_receiver),
        )
        // Yearly
        .route(
            "/api/cards/stats/transfer/yearly/by-card/sender",
            get(get_yearly_transfer_amount_by_card_sender),
        )
        .route(
            "/api/cards/stats/transfer/yearly/by-card/receiver",
            get(get_yearly_transfer_amount_by_card_receiver),
        )
        .route(
            "/api/cards/stats/withdraw/monthly/by-card",
            get(get_monthly_withdraw_amount_by_card),
        )
        .route(
            "/api/cards/stats/withdraw/yearly/by-card",
            get(get_yearly_withdraw_amount_by_card),
        )
        .route("/api/cards/dashboard", get(get_card_dashboard))
        .route(
            "/api/cards/dashboard/{card_number}",
            get(get_card_dashboard_by_card_number),
        )
        .route_layer(middleware::from_fn(jwt::auth))
        .layer(Extension(app_state.di_container.card_clients.clone()))
        .layer(Extension(app_state.jwt_config.clone()))
}
