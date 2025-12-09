use tauri::command;

use crate::{
    domain::{
        requests::user::{
            list::FindAllUser,
            trashed::{delete::DeletePermanentUser, restore::RestoreUserTrashed},
        },
        response::user::{
            ApiResponsePaginationUserDeleteAt, ApiResponseUser, ApiResponseUserAll,
            ApiResponseUserDelete,
        },
    },
    service::user::user_trashed_service::UserTrashedService,
};

#[command]
pub async fn find_all_users_trashed(
    access_token: String,
    req: FindAllUser,
) -> Result<ApiResponsePaginationUserDeleteAt, String> {
    let service = UserTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_users_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_user_trashed(
    access_token: String,
    req: RestoreUserTrashed,
) -> Result<ApiResponseUser, String> {
    let service = UserTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_user_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_user(
    access_token: String,
    req: DeletePermanentUser,
) -> Result<ApiResponseUserDelete, String> {
    let service = UserTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_user_all_trashed(access_token: String) -> Result<ApiResponseUserAll, String> {
    let service = UserTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_user_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_user(access_token: String) -> Result<ApiResponseUserAll, String> {
    let service = UserTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_user(&access_token)
        .await
        .map_err(|e| e.to_string())
}
