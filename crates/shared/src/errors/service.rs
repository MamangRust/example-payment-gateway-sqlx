use crate::errors::repository::RepositoryError;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repo(#[from] RepositoryError),

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

    #[error("Invalid Token")]
    InvalidTokenType,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Custom error: {0}")]
    Custom(String),
}
