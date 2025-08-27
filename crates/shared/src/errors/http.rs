use crate::errors::{
    errors::ErrorResponse, grpc::AppErrorGrpc, repository::RepositoryError, service::ServiceError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppErrorHttp(pub AppErrorGrpc);

impl IntoResponse for AppErrorHttp {
    fn into_response(self) -> Response {
        let (status, msg) = match self.0 {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => {
                    (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
                }

                ServiceError::Validation(errors) => {
                    let error_msg = format!("Validation failed: {errors:?}");
                    (StatusCode::BAD_REQUEST, error_msg)
                }

                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
                    RepositoryError::Conflict(msg) => (StatusCode::CONFLICT, msg),
                    RepositoryError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg),
                    RepositoryError::ForeignKey(msg) => (
                        StatusCode::BAD_REQUEST,
                        format!("Foreign key violation: {msg}"),
                    ),
                    RepositoryError::Sqlx(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    ),
                    RepositoryError::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                },

                ServiceError::Bcrypt(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal authentication error".to_string(),
                ),

                ServiceError::Jwt(err) => (StatusCode::UNAUTHORIZED, format!("JWT error: {err}")),

                ServiceError::TokenExpired => {
                    (StatusCode::UNAUTHORIZED, "Token has expired".to_string())
                }

                ServiceError::InvalidTokenType => {
                    (StatusCode::UNAUTHORIZED, "Invalid token type".to_string())
                }

                ServiceError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),

                ServiceError::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            },
            AppErrorGrpc::Unhandled(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled error: {msg}"),
            ),
        };

        let body = Json(ErrorResponse {
            status: "error".to_string(),
            message: msg,
        });

        (status, body).into_response()
    }
}
