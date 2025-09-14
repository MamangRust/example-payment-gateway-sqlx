use tauri::command;

use crate::{
    domain::{
        requests::topup::{
            create::CreateTopup, findbycardnumber::FindByCardNumberTopup, findbyid::FindByIdTopup,
            list::FindAllTopup, trashedtopup::TrashedTopup, update::UpdateTopup,
        },
        response::topup::{
            ApiResponsePaginationTopup, ApiResponsePaginationTopupDeleteAt, ApiResponseTopup,
            ApiResponseTopupMonthAmount, ApiResponseTopupMonthMethod,
            ApiResponseTopupMonthStatusFailed, ApiResponseTopupMonthStatusSuccess,
            ApiResponseTopupYearAmount, ApiResponseTopupYearMethod,
            ApiResponseTopupYearStatusFailed, ApiResponseTopupYearStatusSuccess,
        },
    },
    service::topup::topup_service::TopupService,
};

#[command]
pub async fn find_month_status_success(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTopupMonthStatusSuccess, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_success(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupYearStatusSuccess, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_success(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseTopupMonthStatusFailed, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_failed(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupYearStatusFailed, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_failed(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_success_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTopupMonthStatusSuccess, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_success_by_card(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_success_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupYearStatusSuccess, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_success_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_status_failed_by_card_number(
    access_token: String,
    year: u32,
    month: u32,
    card_number: String,
) -> Result<ApiResponseTopupMonthStatusFailed, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_status_failed_by_card(&access_token, year, month, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_status_failed_by_card_number(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupYearStatusFailed, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_status_failed_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupMonthMethod, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_topup_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_topup_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupYearMethod, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_topup_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupMonthAmount, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_topup_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_topup_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseTopupYearAmount, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_topup_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_method_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupMonthMethod, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_topup_method_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_topup_method_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupYearMethod, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_topup_method_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_topup_amount_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupMonthAmount, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_month_topup_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_topup_amount_by_card(
    access_token: String,
    year: u32,
    card_number: String,
) -> Result<ApiResponseTopupYearAmount, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_year_topup_amount_by_card(&access_token, year, &card_number)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_topups(
    access_token: String,
    req: FindAllTopup,
) -> Result<ApiResponsePaginationTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_all_topups(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_topup(
    access_token: String,
    req: FindByIdTopup,
) -> Result<ApiResponseTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_topup(
    access_token: String,
    req: FindAllTopup,
) -> Result<ApiResponsePaginationTopupDeleteAt, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_card_number_topup(
    access_token: String,
    req: FindByCardNumberTopup,
) -> Result<ApiResponseTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .find_by_card_number_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_topup(
    access_token: String,
    req: CreateTopup,
) -> Result<ApiResponseTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .create_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_topup(
    access_token: String,
    req: UpdateTopup,
) -> Result<ApiResponseTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .update_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_topup(
    access_token: String,
    req: TrashedTopup,
) -> Result<ApiResponseTopup, String> {
    let service = TopupService::new("http://localhost:5000/api".to_string());

    service
        .trashed_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
