use tauri::command;

use crate::{
    domain::{
        requests::transfer::{
            create::CreateTransfer, findbyid::FindByIdTransfer, list::FindAllTransfer,
            transferfrom::TransferFrom, transferto::TransferTo, trashedtransfer::TrashedTransfer,
            update::UpdateTransfer,
        },
        response::transfer::{
            ApiResponsePaginationTransfer, ApiResponsePaginationTransferDeleteAt,
            ApiResponseTransfer, ApiResponseTransferMonthAmount,
            ApiResponseTransferMonthStatusFailed, ApiResponseTransferMonthStatusSuccess,
            ApiResponseTransferYearAmount, ApiResponseTransferYearStatusFailed,
            ApiResponseTransferYearStatusSuccess, ApiResponseTransfers,
        },
    },
    service::transfer::transfer_service::TransferService,
};

#[command]
pub async fn find_month_status_success_transfer(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTransferMonthStatusSuccess, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_success(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_transfer(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransferYearStatusSuccess, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_success(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_transfer(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTransferMonthStatusFailed, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_failed(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_transfer(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransferYearStatusFailed, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_failed(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_success_transfer_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTransferMonthStatusSuccess, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_success_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_transfer_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferYearStatusSuccess, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_success_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_transfer_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTransferMonthStatusFailed, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_status_failed_by_card_number(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_transfer_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferYearStatusFailed, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_status_failed_by_card_number(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_amount_transfer(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransferMonthAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_transfer_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transfer_amount_transfer(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTransferYearAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_transfer_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_amount_by_sender(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferMonthAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_transfer_amount_by_sender(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transfer_amount_by_sender(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferYearAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_transfer_amount_by_sender(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_transfer_amount_by_receiver(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferMonthAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_month_transfer_amount_by_receiver(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_transfer_amount_by_receiver(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTransferYearAmount, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .find_year_transfer_amount_by_receiver(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_transfers(
    access_token: String,
    req: FindAllTransfer,
) -> Result<ApiResponsePaginationTransfer, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .find_all_transfers(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_transfer(
    access_token: String,
    req: FindByIdTransfer,
) -> Result<ApiResponseTransfer, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_transfer_from(
    access_token: String,
    req: TransferFrom,
) -> Result<ApiResponseTransfers, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .find_by_transfer_from(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_transfer_to(
    access_token: String,
    req: TransferTo,
) -> Result<ApiResponseTransfers, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .find_by_transfer_to(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_transfer(
    access_token: String,
    req: FindAllTransfer,
) -> Result<ApiResponsePaginationTransferDeleteAt, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_transfer(
    access_token: String,
    req: CreateTransfer,
) -> Result<ApiResponseTransfer, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .create_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_transfer(
    access_token: String,
    req: UpdateTransfer,
) -> Result<ApiResponseTransfer, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());

    service
        .update_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_transfer(
    access_token: String,
    req: TrashedTransfer,
) -> Result<ApiResponseTransfer, String> {
    let service = TransferService::new("http://localhost:5000/api".to_string());
    service
        .trashed_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
