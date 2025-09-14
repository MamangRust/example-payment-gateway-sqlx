use tauri::command;

use crate::{
    domain::{
        requests::card::{
            cardnumber::FindByCardNumber, create::CreateCard, findid::FindByIdCard,
            list::FindAllCard, update::UpdateCard, user::FindByUser,
        },
        response::card::{
            ApiResponseCard, ApiResponseDashboardCard, ApiResponseDashboardCardNumber,
            ApiResponseMonthlyAmount, ApiResponseMonthlyBalance, ApiResponsePaginationCard,
            ApiResponsePaginationCardDeleteAt, ApiResponseYearlyAmount, ApiResponseYearlyBalance,
        },
    },
    service::card::card_service::CardService,
};

#[command]
pub async fn find_dashboard(access_token: String) -> Result<ApiResponseDashboardCard, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_dashboard(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_dashboard_by_card_number(
    access_token: String,
    card_number: String,
) -> Result<ApiResponseDashboardCardNumber, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_dashboard_by_card_number(&access_token, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_balance_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyBalance, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_balance(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_balance_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyBalance, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_balance(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_topup_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_topup_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .findyear_topup_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_withdraw_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_withdraw_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_withdraw_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .findyear_withdraw_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_sender_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transfer_sender_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_transfer_sender_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transfer_sender_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_receiver_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transfer_receiver_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_transfer_receiver_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transfer_receiver_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transaction_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_transaction_amount_card(
    access_token: String,
    year: i32,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transaction_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_balance_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyBalance, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_balance_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_balance_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyBalance, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_balance_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_topup_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_topup_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .findyear_topup_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_withdraw_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_withdraw_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_withdraw_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .findyear_withdraw_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_sender_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transfer_sender_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transfer_sender_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transfer_sender_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_receiver_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transfer_receiver_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transfer_receiver_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transfer_receiver_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseMonthlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_month_transaction_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn findyear_transaction_amount_by_card_card(
    access_token: String,
    year: i32,
    card_number: String,
) -> Result<ApiResponseYearlyAmount, String> {
    let card_service = CardService::new("http://localhost:5000/api".to_string());
    card_service
        .find_year_transaction_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_card(
    access_token: String,
    req: FindAllCard,
) -> Result<ApiResponsePaginationCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .find_all_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_card(
    access_token: String,
    req: FindByIdCard,
) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_user(
    access_token: String,
    req: FindByUser,
) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .find_by_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_card_number(
    access_token: String,
    req: FindByCardNumber,
) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .find_by_card_number(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_active_card(
    access_token: String,
    req: FindAllCard,
) -> Result<ApiResponsePaginationCardDeleteAt, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .find_active_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_card(access_token: String, req: CreateCard) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .create_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_card(access_token: String, req: UpdateCard) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .update_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_card(
    access_token: String,
    req: FindByIdCard,
) -> Result<ApiResponseCard, String> {
    let service = CardService::new("http://localhost:5000/api".to_string());

    service
        .trashed_card(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
