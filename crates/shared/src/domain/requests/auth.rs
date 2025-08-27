use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AuthRequest {
    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 2, message = "First name must be at least 2 characters"))]
    pub firstname: String,

    #[validate(length(min = 2, message = "Last name must be at least 2 characters"))]
    pub lastname: String,

    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,

    #[validate(length(min = 6, message = "Confirm password minimal 6 karakter"))]
    pub confirm_password: String,
}
