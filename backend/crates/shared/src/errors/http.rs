use crate::errors::{
    error::ErrorResponse, grpc::AppErrorGrpc, repository::RepositoryError, service::ServiceError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info, warn};

#[derive(Debug)]
pub enum HttpError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    ServiceUnavailable(String),
    Internal(String),
    Forbidden(String),
}

impl From<AppErrorGrpc> for HttpError {
    fn from(err: AppErrorGrpc) -> Self {
        match err {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => {
                    HttpError::Unauthorized("Invalid credentials".to_string())
                }

                ServiceError::Validation(errors) => {
                    HttpError::BadRequest(format!("Validation failed: {errors:?}"))
                }

                ServiceError::Forbidden(msg) => HttpError::Forbidden(msg),

                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => HttpError::NotFound("Not found".into()),
                    RepositoryError::Conflict(msg) => HttpError::Conflict(msg),
                    RepositoryError::AlreadyExists(msg) => HttpError::Conflict(msg),
                    RepositoryError::ForeignKey(msg) => {
                        HttpError::BadRequest(format!("Foreign key violation: {msg}"))
                    }
                    _ => HttpError::Internal("Repository error".into()),
                },

                ServiceError::Jwt(err) => HttpError::Unauthorized(format!("JWT error: {err}")),

                ServiceError::InternalServerError(msg) | ServiceError::Custom(msg) => {
                    HttpError::Internal(msg)
                }

                ServiceError::Bcrypt(_) => {
                    HttpError::Internal("Internal authentication error".into())
                }

                ServiceError::TokenExpired => HttpError::Unauthorized("Token expired".into()),

                ServiceError::InvalidTokenType => {
                    HttpError::Unauthorized("Invalid token type".into())
                }
                _ => HttpError::Internal("Unknown service error".into()),
            },

            AppErrorGrpc::Unhandled(msg) => HttpError::Internal(msg),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, msg, log_level) = match self {
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, "warn"),
            HttpError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg, "warn"),
            HttpError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg, "warn"),
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, "info"),
            HttpError::Conflict(msg) => (StatusCode::CONFLICT, msg, "warn"),
            HttpError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg, "error"),
            HttpError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg, "error"),
        };

        match log_level {
            "error" => error!("HTTP {}: {}", status, msg),
            "warn" => warn!("HTTP {}: {}", status, msg),
            "info" => info!("HTTP {}: {}", status, msg),
            _ => error!("HTTP {}: {}", status, msg),
        }

        let body = Json(ErrorResponse {
            status: "error".into(),
            message: msg,
        });

        (status, body).into_response()
    }
}
