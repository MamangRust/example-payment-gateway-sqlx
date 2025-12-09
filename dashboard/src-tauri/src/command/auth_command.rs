use crate::{
    domain::requests::auth::{login::LoginRequest, register::RegisterRequest},
    model::auth::{
        ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister,
    },
    service::auth_service::AuthService,
};

#[tauri::command]
pub async fn login(req: LoginRequest) -> Result<ApiResponseLogin, String> {
    let auth_service = AuthService::new("http://localhost:5000/api".to_string());

    auth_service.login(&req).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register(req: RegisterRequest) -> Result<ApiResponseRegister, String> {
    let auth_service = AuthService::new("http://localhost:5000/api".to_string());
    auth_service.register(&req).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_me(access_token: &str) -> Result<ApiResponseGetMe, String> {
    let auth_service = AuthService::new("http://localhost:5000/api".to_string());
    auth_service
        .get_me(access_token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn myrefresh_token(
    access_token: &str,
    refresh_token: &str,
) -> Result<ApiResponseRefreshToken, String> {
    let auth_service = AuthService::new("http://localhost:5000/api".to_string());
    auth_service
        .refresh_access_token(&access_token, &refresh_token)
        .await
        .map_err(|e| e.to_string())
}
