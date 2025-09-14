use tauri::command;

use crate::{
    domain::{
        requests::transfer::{
            list::FindAllTransfer,
            trashed::{delete::DeletePermanentTransfer, restore::RestoreTransferTrashed},
        },
        response::transfer::{
            ApiResponsePaginationTransferDeleteAt, ApiResponseTransfer, ApiResponseTransferAll,
            ApiResponseTransferDelete,
        },
    },
    service::transfer::transfer_trashed_service::TransferTrashedService,
};

#[command]
pub async fn find_all_transfers_trashed(
    access_token: String,
    req: FindAllTransfer,
) -> Result<ApiResponsePaginationTransferDeleteAt, String> {
    let service = TransferTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_transfers_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_transfer_trashed(
    access_token: String,
    req: RestoreTransferTrashed,
) -> Result<ApiResponseTransfer, String> {
    let service = TransferTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_transfer_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_transfer(
    access_token: String,
    req: DeletePermanentTransfer,
) -> Result<ApiResponseTransferDelete, String> {
    let service = TransferTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_transfer(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_transfer_all_trashed(
    access_token: String,
) -> Result<ApiResponseTransferAll, String> {
    let service = TransferTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_transfer_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_transfer(
    access_token: String,
) -> Result<ApiResponseTransferAll, String> {
    let service = TransferTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_transfer(&access_token)
        .await
        .map_err(|e| e.to_string())
}
