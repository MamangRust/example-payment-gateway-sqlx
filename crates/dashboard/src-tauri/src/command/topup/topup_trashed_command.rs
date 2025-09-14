use tauri::command;

use crate::{
    domain::{
        requests::topup::{
            list::FindAllTopup,
            trashed::{delete::DeletePermanentTopup, restore::RestoreTopupTrashed},
        },
        response::topup::{
            ApiResponsePaginationTopupDeleteAt, ApiResponseTopup, ApiResponseTopupAll,
            ApiResponseTopupDelete,
        },
    },
    service::topup::topup_trashed_service::TopupTrashedService,
};

#[command]
pub async fn find_all_topups_trashed(
    access_token: String,
    req: FindAllTopup,
) -> Result<ApiResponsePaginationTopupDeleteAt, String> {
    let service = TopupTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_topups_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_topup_trashed(
    access_token: String,
    req: RestoreTopupTrashed,
) -> Result<ApiResponseTopup, String> {
    let service = TopupTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_topup_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_topup(
    access_token: String,
    req: DeletePermanentTopup,
) -> Result<ApiResponseTopupDelete, String> {
    let service = TopupTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_topup(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_topup_all_trashed(
    access_token: String,
) -> Result<ApiResponseTopupAll, String> {
    let service = TopupTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_topup_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_topup(
    access_token: String,
) -> Result<ApiResponseTopupAll, String> {
    let service = TopupTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_topup(&access_token)
        .await
        .map_err(|e| e.to_string())
}
