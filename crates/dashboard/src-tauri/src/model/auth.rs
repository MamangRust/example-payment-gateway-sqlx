use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseLogin {
    pub status: String,
    pub message: String,
    pub data: Option<TokenResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseRegister {
    pub status: String,
    pub message: String,
    pub data: Option<UserResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseRefreshToken {
    pub status: String,
    pub message: String,
    pub data: Option<TokenResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseGetMe {
    pub status: String,
    pub message: String,
    pub data: Option<UserResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
