use tauri::command;

use crate::{
    domain::{
        requests::user::{
            create::CreateUser, findbyid::FindByIdUser, list::FindAllUser,
            trashed::list::FindAllUserTrashed, trasheduser::TrashedUser, update::UpdateUser,
        },
        response::user::{
            ApiResponsePaginationUser, ApiResponsePaginationUserDeleteAt, ApiResponseUser,
        },
    },
    service::user::user_service::UserService,
};

#[command]
pub async fn find_all_users(
    access_token: String,
    req: FindAllUser,
) -> Result<ApiResponsePaginationUser, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .find_all_users(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_id_user(
    access_token: String,
    req: FindByIdUser,
) -> Result<ApiResponseUser, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .find_by_id(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn find_by_active(
    access_token: String,
    req: FindAllUserTrashed,
) -> Result<ApiResponsePaginationUserDeleteAt, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .find_by_active(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn create_user(access_token: String, req: CreateUser) -> Result<ApiResponseUser, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .create_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_user(access_token: String, req: UpdateUser) -> Result<ApiResponseUser, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .update_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn trashed_user(
    access_token: String,
    req: TrashedUser,
) -> Result<ApiResponseUser, String> {
    let service = UserService::new("http://localhost:5000/api".to_string());

    service
        .trashed_user(&access_token, req)
        .await
        .map_err(|e| e.to_string())
}
