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
pub struct AppErrorHttp(pub AppErrorGrpc);

impl IntoResponse for AppErrorHttp {
    fn into_response(self) -> Response {
        let (status, msg, log_level) = match self.0 {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => {
                    warn!("🔐 Invalid credentials attempt");
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid credentials".to_string(),
                        "warn",
                    )
                }
                ServiceError::Validation(errors) => {
                    warn!("📝 Validation failed: {errors:?}");
                    let error_msg = format!("Validation failed: {errors:?}");
                    (StatusCode::BAD_REQUEST, error_msg, "warn")
                }
                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => {
                        info!("🔍 Resource not found");
                        (StatusCode::NOT_FOUND, "Not found".to_string(), "info")
                    }
                    RepositoryError::Conflict(msg) => {
                        warn!("⚡ Conflict detected: {}", msg);
                        (StatusCode::CONFLICT, msg, "warn")
                    }
                    RepositoryError::AlreadyExists(msg) => {
                        warn!("📦 Resource already exists: {}", msg);
                        (StatusCode::CONFLICT, msg, "warn")
                    }
                    RepositoryError::ForeignKey(msg) => {
                        warn!("🔗 Foreign key violation: {}", msg);
                        (
                            StatusCode::BAD_REQUEST,
                            format!("Foreign key violation: {msg}"),
                            "warn",
                        )
                    }
                    RepositoryError::Sqlx(err) => {
                        error!("💾 Database error: {}", err);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Database error".to_string(),
                            "error",
                        )
                    }
                    RepositoryError::Custom(msg) => {
                        error!("⚙️ Custom repository error: {}", msg);
                        (StatusCode::INTERNAL_SERVER_ERROR, msg, "error")
                    }
                },
                ServiceError::Bcrypt(err) => {
                    error!("🔒 Bcrypt error: {}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal authentication error".to_string(),
                        "error",
                    )
                }
                ServiceError::Jwt(err) => {
                    warn!("🎫 JWT error: {}", err);
                    (
                        StatusCode::UNAUTHORIZED,
                        format!("JWT error: {err}"),
                        "warn",
                    )
                }
                ServiceError::TokenExpired => {
                    warn!("⏰ Token expired");
                    (
                        StatusCode::UNAUTHORIZED,
                        "Token has expired".to_string(),
                        "warn",
                    )
                }
                ServiceError::InvalidTokenType => {
                    warn!("🎫 Invalid token type");
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid token type".to_string(),
                        "warn",
                    )
                }
                ServiceError::InternalServerError(msg) => {
                    error!("🔥 Internal server error: {}", msg);
                    (StatusCode::INTERNAL_SERVER_ERROR, msg, "error")
                }
                ServiceError::Custom(msg) => {
                    error!("⚙️ Custom service error: {}", msg);
                    (StatusCode::INTERNAL_SERVER_ERROR, msg, "error")
                }
                ServiceError::NotFound(msg) => {
                    info!("🔍 Not found: {}", msg);
                    (StatusCode::NOT_FOUND, msg, "info")
                }
            },
            AppErrorGrpc::Unhandled(msg) => {
                error!("💥 Unhandled error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {msg}"),
                    "error",
                )
            }
        };

        match log_level {
            "error" => error!("🚨 HTTP Error {}: {}", status, msg),
            "warn" => warn!("⚠️ HTTP Warning {}: {}", status, msg),
            "info" => info!("ℹ️ HTTP Info {}: {}", status, msg),
            _ => error!("🚨 HTTP Error {}: {}", status, msg),
        }

        let body = Json(ErrorResponse {
            status: "error".to_string(),
            message: msg,
        });

        (status, body).into_response()
    }
}

impl From<AppErrorGrpc> for AppErrorHttp {
    fn from(error: AppErrorGrpc) -> Self {
        AppErrorHttp(error)
    }
}

impl From<ServiceError> for AppErrorHttp {
    fn from(error: ServiceError) -> Self {
        AppErrorHttp(AppErrorGrpc::Service(error))
    }
}

impl From<RepositoryError> for AppErrorHttp {
    fn from(error: RepositoryError) -> Self {
        AppErrorHttp(AppErrorGrpc::Service(ServiceError::Repo(error)))
    }
}
