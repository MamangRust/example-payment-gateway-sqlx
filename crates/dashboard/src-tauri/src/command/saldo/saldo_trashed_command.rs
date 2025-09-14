use tauri::command;

use crate::{
    domain::{
        requests::saldo::{
            list::FindAllSaldo,
            trashed::{delete::DeletePermanentSaldo, restore::RestoreSaldoTrashed},
        },
        response::saldo::{
            ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseSaldoAll,
            ApiResponseSaldoDelete,
        },
    },
    service::saldo::saldo_trashed_service::SaldoTrashedService,
};

#[command]
pub async fn find_all_saldos_trashed(
    access_token: String,
    req: FindAllSaldo,
) -> Result<ApiResponsePaginationSaldoDeleteAt, String> {
    let service = SaldoTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_saldos_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_saldo_trashed(
    access_token: String,
    req: RestoreSaldoTrashed,
) -> Result<ApiResponseSaldo, String> {
    let service = SaldoTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_saldo_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_saldo(
    access_token: String,
    req: DeletePermanentSaldo,
) -> Result<ApiResponseSaldoDelete, String> {
    let service = SaldoTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_saldo(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_saldo_all_trashed(
    access_token: String,
) -> Result<ApiResponseSaldoAll, String> {
    let service = SaldoTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_saldo_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_saldo(
    access_token: String,
) -> Result<ApiResponseSaldoAll, String> {
    let service = SaldoTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_saldo(&access_token)
        .await
        .map_err(|e| e.to_string())
}
