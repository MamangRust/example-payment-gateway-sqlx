use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRefreshToken {
    #[validate(range(min = 1, message = "User ID wajib >= 1"))]
    pub user_id: i32,

    #[validate(length(min = 1, message = "Token wajib diisi"))]
    pub token: String,

    #[validate(length(min = 1, message = "ExpiresAt wajib diisi"))]
    pub expires_at: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRefreshToken {
    #[validate(range(min = 1, message = "User ID wajib >= 1"))]
    pub user_id: i32,

    #[validate(length(min = 1, message = "Token wajib diisi"))]
    pub token: String,

    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token wajib diisi"))]
    pub refresh_token: String,
}
