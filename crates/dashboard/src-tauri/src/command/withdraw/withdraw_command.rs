use tauri::command;
use tracing::info;

use crate::{
    domain::{
        requests::withdraw::{
            create::CreateWithdraw, findbycardnumber::FindByCardNumberWithdraw,
            findbyid::FindByIdWithdraw, list::FindAllWithdraw, trashedwithdraw::TrashedWithdraw,
            update::UpdateWithdraw,
        },
        response::withdraw::{
            ApiResponsePaginationWithdraw, ApiResponsePaginationWithdrawDeleteAt,
            ApiResponseWithdraw, ApiResponseWithdrawMonthAmount,
            ApiResponseWithdrawMonthStatusFailed, ApiResponseWithdrawMonthStatusSuccess,
            ApiResponseWithdrawYearAmount, ApiResponseWithdrawYearStatusFailed,
            ApiResponseWithdrawYearStatusSuccess,
        },
    },
    service::withdraw::withdraw_service::WithdrawService,
};

#[command]
pub async fn find_month_status_success_withdraw(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseWithdrawMonthStatusSuccess, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_success(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_withdraw(
    access_token: String,
    year: u32,
) -> Result<ApiResponseWithdrawYearStatusSuccess, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_success(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_withdraw(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseWithdrawMonthStatusFailed, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_failed(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_withdraw(
    access_token: String,
    year: u32,
) -> Result<ApiResponseWithdrawYearStatusFailed, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_failed(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_success_withdraw_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawMonthStatusSuccess, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_success_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_withdraw_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawYearStatusSuccess, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_success_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_withdraw_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawMonthStatusFailed, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_failed_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_withdraw_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawYearStatusFailed, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_failed_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_withdraw_amount_withdraw(
    access_token: String,
    year: u32,
) -> Result<ApiResponseWithdrawMonthAmount, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_withdraw_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_withdraw_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseWithdrawYearAmount, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_withdraw_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_withdraw_amount_by_card_withdraw(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawMonthAmount, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_month_withdraw_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_withdraw_amount_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseWithdrawYearAmount, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    service
        .find_year_withdraw_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_withdraws(
    access_token: String,
    req: FindAllWithdraw,
) -> Result<ApiResponsePaginationWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    let response = service
        .find_all_withdraws(&access_token, req)
        .await
        .map_err(|e| e.to_string());

    info!("response = {:?}", response);

    response
}

#[command]
pub async fn find_by_id_withdraw(
    access_token: String,
    req: FindByIdWithdraw,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_card_number_withdraw(
    access_token: String,
    card_number: String,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());
    let req = FindByCardNumberWithdraw { card_number };
    service
        .find_by_card_number_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_withdraw(
    access_token: String,
    req: FindAllWithdraw,
) -> Result<ApiResponsePaginationWithdrawDeleteAt, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_withdraw(
    access_token: String,
    req: CreateWithdraw,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    service
        .create_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_withdraw(
    access_token: String,
    req: UpdateWithdraw,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    service
        .update_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_withdraw(
    access_token: String,
    req: TrashedWithdraw,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawService::new("http://localhost:5000/api".to_string());

    service
        .trashed_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
