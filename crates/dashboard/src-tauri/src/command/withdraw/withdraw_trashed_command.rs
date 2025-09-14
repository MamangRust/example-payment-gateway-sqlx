use tauri::command;

use crate::{
    domain::{
        requests::withdraw::{
            list::FindAllWithdraw,
            trashed::{delete::DeletePermanentWithdraw, restore::RestoreWithdrawTrashed},
        },
        response::withdraw::{
            ApiResponsePaginationWithdrawDeleteAt, ApiResponseWithdraw, ApiResponseWithdrawAll,
            ApiResponseWithdrawDelete,
        },
    },
    service::withdraw::withdraw_trashed_service::WithdrawTrashedService,
};

#[command]
pub async fn find_all_withdraws_trashed(
    access_token: String,
    req: FindAllWithdraw,
) -> Result<ApiResponsePaginationWithdrawDeleteAt, String> {
    let service = WithdrawTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_withdraws_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_withdraw_trashed(
    access_token: String,
    req: RestoreWithdrawTrashed,
) -> Result<ApiResponseWithdraw, String> {
    let service = WithdrawTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_withdraw_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_withdraw(
    access_token: String,
    req: DeletePermanentWithdraw,
) -> Result<ApiResponseWithdrawDelete, String> {
    let service = WithdrawTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_withdraw(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_withdraw_all_trashed(
    access_token: String,
) -> Result<ApiResponseWithdrawAll, String> {
    let service = WithdrawTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_withdraw_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_withdraw(
    access_token: String,
) -> Result<ApiResponseWithdrawAll, String> {
    let service = WithdrawTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_withdraw(&access_token)
        .await
        .map_err(|e| e.to_string())
}
