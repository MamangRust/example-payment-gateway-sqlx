use crate::errors::repository::RepositoryError;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repo(#[from] RepositoryError),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Validation failed: {0:?}")]
    Validation(Vec<String>),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),

    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token type")]
    InvalidTokenType,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Custom error: {0}")]
    Custom(String),
}
