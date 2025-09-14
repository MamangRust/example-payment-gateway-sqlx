use tauri::command;

use crate::{
    domain::{
        requests::transaction::{
            list::FindAllTransaction,
            trashed::{delete::DeletePermanentTransaction, restore::RestoreTransactionTrashed},
        },
        response::transaction::{
            ApiResponsePaginationTransactionDeleteAt, ApiResponseTransaction,
            ApiResponseTransactionAll, ApiResponseTransactionDelete,
        },
    },
    service::transaction::transaction_trashed_service::TransactionTrashedService,
};

#[command]
pub async fn find_all_transactions_trashed(
    access_token: String,
    req: FindAllTransaction,
) -> Result<ApiResponsePaginationTransactionDeleteAt, String> {
    let service = TransactionTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_transactions_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_transaction_trashed(
    access_token: String,
    req: RestoreTransactionTrashed,
) -> Result<ApiResponseTransaction, String> {
    let service = TransactionTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_transaction_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_transaction(
    access_token: String,
    req: DeletePermanentTransaction,
) -> Result<ApiResponseTransactionDelete, String> {
    let service = TransactionTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_transaction(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_transaction_all_trashed(
    access_token: String,
) -> Result<ApiResponseTransactionAll, String> {
    let service = TransactionTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_trasnaction_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_transaction(
    access_token: String,
) -> Result<ApiResponseTransactionAll, String> {
    let service = TransactionTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_transaction(&access_token)
        .await
        .map_err(|e| e.to_string())
}
