use tauri::command;

use crate::{
    domain::{
        requests::role::{
            create::CreateRole, findbyid::FindByIdRole, list::FindAllRole,
            trashed::list::FindAllRoleTrashed, trashedrole::TrashedRole, update::UpdateRole,
        },
        response::role::{
            ApiResponsePaginationRole, ApiResponsePaginationRoleDeleteAt, ApiResponseRole,
        },
    },
    service::role::role_service::RoleService,
};

#[command]
pub async fn find_all_roles(
    access_token: String,
    req: FindAllRole,
) -> Result<ApiResponsePaginationRole, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .find_all_roles(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_role(
    access_token: String,
    req: FindByIdRole,
) -> Result<ApiResponseRole, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active_role(
    access_token: String,
    req: FindAllRoleTrashed,
) -> Result<ApiResponsePaginationRoleDeleteAt, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_role(access_token: String, req: CreateRole) -> Result<ApiResponseRole, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .create_role(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_role(access_token: String, req: UpdateRole) -> Result<ApiResponseRole, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .update_role(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_role(
    access_token: String,
    req: TrashedRole,
) -> Result<ApiResponseRole, String> {
    let service = RoleService::new("http://localhost:5000/api".to_string());

    service
        .trashed_role(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
