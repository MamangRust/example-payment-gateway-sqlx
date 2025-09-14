use tauri::command;

use crate::{
    domain::{
        requests::merchant::{
            create::CreateMerchant,
            findbyapikey::FindByApiKeyMerchant,
            findbyid::FindByIdMerchant,
            findmerchantuser::FindMerchantUser,
            list::{FindAllMerchant, FindAllMerchantTransaction, FindAllMerchantTransactionApiKey},
            trashedmerchant::FindTrashedMerchant,
            update::UpdateMerchant,
        },
        response::merchant::{
            ApiResponseMerchant, ApiResponseMerchantMonthlyAmount,
            ApiResponseMerchantMonthlyPaymentMethod, ApiResponseMerchantMonthlyTotalAmount,
            ApiResponseMerchantYearlyAmount, ApiResponseMerchantYearlyPaymentMethod,
            ApiResponseMerchantYearlyTotalAmount, ApiResponsePaginationMerchant,
            ApiResponsePaginationMerchantDeleteAt, ApiResponsePaginationMerchantTransaction,
            ApiResponsesMerchant,
        },
    },
    service::merchant::merchant_service::MerchantService,
};

#[command]
pub async fn find_month_payment_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMerchantMonthlyPaymentMethod, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_month_payment_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_payment_method(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMerchantYearlyPaymentMethod, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_year_payment_method(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMerchantMonthlyAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_month_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMerchantYearlyAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_year_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_total_amount(
    access_token: String,
    year: u32,
    month: u32,
) -> Result<ApiResponseMerchantMonthlyTotalAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_month_total_amount(&access_token, year, month)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_total_amount(
    access_token: String,
    year: u32,
) -> Result<ApiResponseMerchantYearlyTotalAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_year_total_amount(&access_token, year)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_payment_method_by_merchant(
    access_token: String,
    year: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantMonthlyPaymentMethod, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_month_payment_method_by_merchant(&access_token, year, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_payment_method_by_merchant(
    access_token: String,
    year: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantYearlyPaymentMethod, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_year_payment_method_by_merchant(&access_token, year, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_amount_by_merchant(
    access_token: String,
    year: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantMonthlyAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_month_amount_by_merchant(&access_token, year, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_amount_by_merchant(
    access_token: String,
    year: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantYearlyAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_year_amount_by_merchant(&access_token, year, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_month_total_amount_by_merchant(
    access_token: String,
    year: u32,
    month: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantMonthlyTotalAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_month_total_amount_by_merchant(&access_token, year, month, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_year_total_amount_by_merchant(
    access_token: String,
    year: u32,
    merchant_id: u32,
) -> Result<ApiResponseMerchantYearlyTotalAmount, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_year_total_amount_by_merchant(&access_token, year, merchant_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_merchants(
    access_token: String,
    req: FindAllMerchant,
) -> Result<ApiResponsePaginationMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_all_merchants(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_transactions(
    access_token: String,
    req: FindAllMerchant,
) -> Result<ApiResponsePaginationMerchantTransaction, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_all_transactions(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_transactions_by_merchant(
    access_token: String,
    req: FindAllMerchantTransaction,
) -> Result<ApiResponsePaginationMerchantTransaction, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_all_transactions_by_merchant(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_all_transactions_by_api_key(
    access_token: String,
    req: FindAllMerchantTransactionApiKey,
) -> Result<ApiResponsePaginationMerchantTransaction, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_all_transactions_by_api_key(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id(
    access_token: String,
    req: FindByIdMerchant,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_by_id(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_api_key(
    access_token: String,
    req: FindByApiKeyMerchant,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .find_by_api_key(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_merchant_user(
    access_token: String,
    req: FindMerchantUser,
) -> Result<ApiResponsesMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_by_merchant_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_merchant(
    access_token: String,
    req: FindAllMerchant,
) -> Result<ApiResponsePaginationMerchantDeleteAt, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_merchant(
    access_token: String,
    req: CreateMerchant,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .create_merchant(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_merchant(
    access_token: String,
    req: UpdateMerchant,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .update_merchant(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_merchant(
    access_token: String,
    req: FindTrashedMerchant,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantService::new("http://localhost:5000/api".to_string());
    service
        .trashed_merchant(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
