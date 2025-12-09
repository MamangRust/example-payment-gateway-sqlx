use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Foreign key violation: {0}")]
    ForeignKey(String),

    #[error("Custom: {0}")]
    Custom(String),
}
