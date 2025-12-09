use tauri::command;

use crate::{
    domain::{
        requests::merchant::{
            list::FindAllMerchant,
            trashed::{delete::DeletePermanentMerchant, restore::RestoreMerchantTrashed},
        },
        response::merchant::{
            ApiResponseMerchant, ApiResponseMerchantAll, ApiResponseMerchantDelete,
            ApiResponsePaginationMerchantDeleteAt,
        },
    },
    service::merchant::merchant_trashed_service::MerchantTrashedService,
};

#[command]
pub async fn find_all_merchants_trashed(
    access_token: String,
    req: FindAllMerchant,
) -> Result<ApiResponsePaginationMerchantDeleteAt, String> {
    let service = MerchantTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_merchants_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_merchant_trashed(
    access_token: String,
    req: RestoreMerchantTrashed,
) -> Result<ApiResponseMerchant, String> {
    let service = MerchantTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_merchant_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_merchant(
    access_token: String,
    req: DeletePermanentMerchant,
) -> Result<ApiResponseMerchantDelete, String> {
    let service = MerchantTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_merchant(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_merchant_all_trashed(
    access_token: String,
) -> Result<ApiResponseMerchantAll, String> {
    let service = MerchantTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_merchant_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_merchant(
    access_token: String,
) -> Result<ApiResponseMerchantAll, String> {
    let service = MerchantTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_merchant(&access_token)
        .await
        .map_err(|e| e.to_string())
}
