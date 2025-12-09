use tauri::command;

use crate::{
    domain::{
        requests::role::{
            list::FindAllRole,
            trashed::{delete::DeletePermanentRole, restore::RestoreRoleTrashed},
        },
        response::role::{
            ApiResponsePaginationRoleDeleteAt, ApiResponseRole, ApiResponseRoleAll,
            ApiResponseRoleDelete,
        },
    },
    service::role::role_trashed_service::RoleTrashedService,
};

#[command]
pub async fn find_all_roles_trashed(
    access_token: String,
    req: FindAllRole,
) -> Result<ApiResponsePaginationRoleDeleteAt, String> {
    let service = RoleTrashedService::new("http://localhost:5000/api".to_string());
    service
        .find_all_roles_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_role_trashed(
    access_token: String,
    req: RestoreRoleTrashed,
) -> Result<ApiResponseRole, String> {
    let service = RoleTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_role_trashed(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_role(
    access_token: String,
    req: DeletePermanentRole,
) -> Result<ApiResponseRoleDelete, String> {
    let service = RoleTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_role(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn restore_role_all_trashed(access_token: String) -> Result<ApiResponseRoleAll, String> {
    let service = RoleTrashedService::new("http://localhost:5000/api".to_string());
    service
        .restore_role_all_trashed(&access_token)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_permanent_all_role(access_token: String) -> Result<ApiResponseRoleAll, String> {
    let service = RoleTrashedService::new("http://localhost:5000/api".to_string());
    service
        .delete_permanent_all_role(&access_token)
        .await
        .map_err(|e| e.to_string())
}
