use crate::{
    domain::{
        requests::transaction::{
            create::CreateTransaction, findbycardnumber::FindByCardNumberTransaction,
            findbyid::FindByIdTransaction, findbyidmerchant::FindByMerchantTransaction,
            list::FindAllTransaction, trashedtransaction::TrashedTransaction,
            update::UpdateTransaction,
        },
        response::transaction::{
            ApiResponsePaginationTransaction, ApiResponsePaginationTransactionDeleteAt,
            ApiResponseTransaction, ApiResponseTransactionMonthAmount,
            ApiResponseTransactionMonthMethod, ApiResponseTransactionMonthStatusFailed,
            ApiResponseTransactionMonthStatusSuccess, ApiResponseTransactionYearAmount,
            ApiResponseTransactionYearMethod, ApiResponseTransactionYearStatusFailed,
            ApiResponseTransactionYearStatusSuccess, ApiResponseTransactions,
        },
    },
    service::transaction::transaction_service::TransactionService,
};

use tauri::command;

#[command]
pub async fn find_month_status_success_transaction(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTransactionMonthStatusSuccess, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_success(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_transaction(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionYearStatusSuccess, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_success(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_transaction(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTransactionMonthStatusFailed, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_failed(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_transaction(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionYearStatusFailed, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_failed(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_success_transaction_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTransactionMonthStatusSuccess, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_success_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_transaction_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionYearStatusSuccess, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_success_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_transaction_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTransactionMonthStatusFailed, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_failed_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_transaction_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionYearStatusFailed, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_failed_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionMonthMethod, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_transaction_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transaction_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionYearMethod, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_transaction_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionMonthAmount, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_transaction_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transaction_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransactionYearAmount, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_transaction_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_method_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionMonthMethod, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_transaction_method_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transaction_method_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionYearMethod, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_transaction_method_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transaction_amount_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionMonthAmount, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_month_transaction_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transaction_amount_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransactionYearAmount, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_year_transaction_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_transactions_transaction(
    access_token: String,
    req: FindAllTransaction,
) -> Result<ApiResponsePaginationTransaction, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_all_transactions(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_transaction(
    access_token: String,
    req: FindByIdTransaction,
) -> Result<ApiResponseTransaction, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_card_number_transaction(
    access_token: String,
    req: FindByCardNumberTransaction,
) -> Result<ApiResponseTransactions, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_by_card_number_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_merchant_transaction(
    access_token: String,
    req: FindByMerchantTransaction,
) -> Result<ApiResponseTransactions, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_by_merchant_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_transaction(
    access_token: String,
    req: FindAllTransaction,
) -> Result<ApiResponsePaginationTransactionDeleteAt, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_transaction(
    access_token: String,
    req: CreateTransaction,
) -> Result<ApiResponseTransaction, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .create_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_transaction(
    access_token: String,
    req: UpdateTransaction,
) -> Result<ApiResponseTransaction, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .update_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_transaction(
    access_token: String,
    req: TrashedTransaction,
) -> Result<ApiResponseTransaction, String> {
    let service = TransactionService::new("http://localhost:5000/api".to_string());

    service
        .trashed_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
