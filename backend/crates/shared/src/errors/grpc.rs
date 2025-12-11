use crate::errors::{repository::RepositoryError, service::ServiceError};
use thiserror::Error;
use tonic::Status;
use tracing::{error, warn};

#[derive(Debug, Error)]
pub enum AppErrorGrpc {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Unhandled: {0}")]
    Unhandled(String),
}

impl AppErrorGrpc {
    pub fn log(&self) {
        match self {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::NotFound(_) => warn!("⚠️ {}", self),
                ServiceError::InvalidCredentials => warn!("🔐 {}", self),
                ServiceError::TokenExpired => warn!("⏰ {}", self),
                _ => error!("🚨 {}", self),
            },
            AppErrorGrpc::Unhandled(_) => error!("💥 {}", self),
        }
    }
}

impl From<AppErrorGrpc> for Status {
    fn from(err: AppErrorGrpc) -> Self {
        err.log();
        match err {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => {
                    Status::unauthenticated("🔐 Invalid credentials")
                }
                ServiceError::Validation(errors) => {
                    Status::invalid_argument(format!("📝 Validation failed: {errors:#?}"))
                }
                ServiceError::Forbidden(msg) => Status::permission_denied(msg),
                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => Status::not_found("🔍 Resource not found"),
                    RepositoryError::Conflict(msg) => {
                        Status::already_exists(format!("⚡ Conflict: {msg}"))
                    }
                    RepositoryError::AlreadyExists(msg) => {
                        Status::already_exists(format!("📦 Already exists: {msg}"))
                    }
                    RepositoryError::ForeignKey(msg) => {
                        Status::failed_precondition(format!("🔗 Foreign key constraint: {msg}"))
                    }
                    RepositoryError::Sqlx(err) => {
                        error!("💾 Database SQLx error: {err:?}");
                        Status::internal("💾 Database operation failed")
                    }
                    RepositoryError::Custom(msg) => {
                        warn!("⚙️ Custom repository error: {msg}",);
                        Status::internal(format!("⚙️ {msg}"))
                    }
                },
                ServiceError::Bcrypt(err) => {
                    error!("🔒 Bcrypt error: {err:?}");
                    Status::internal("🔒 Password processing error")
                }
                ServiceError::Jwt(err) => {
                    warn!("🎫 JWT error: {err:?}");
                    Status::unauthenticated(format!("🎫 Token error: {err:?}"))
                }
                ServiceError::TokenExpired => Status::unauthenticated("⏰ Token has expired"),
                ServiceError::InvalidTokenType => Status::unauthenticated("🎫 Invalid token type"),
                ServiceError::InternalServerError(msg) => {
                    error!("🔥 Internal server error: {msg}",);
                    Status::internal(format!("🔥 {msg}"))
                }
                ServiceError::Custom(msg) => {
                    warn!("⚙️ Custom service error: {msg}");
                    Status::internal(format!("⚙️ {msg}"))
                }
                ServiceError::NotFound(msg) => {
                    warn!("🔍 Not found: {msg}");
                    Status::not_found(format!("🔍 {msg}"))
                }
            },

            AppErrorGrpc::Unhandled(msg) => {
                error!("💥 Unhandled application error: {msg}");
                Status::internal(format!("💥 Unexpected error: {msg}"))
            }
        }
    }
}

impl From<Status> for AppErrorGrpc {
    fn from(status: Status) -> Self {
        let status_code = status.code();
        let message = status.message().to_string();

        warn!("📡 Received gRPC status: {status_code} - {message}");

        match status.code() {
            tonic::Code::Unauthenticated => AppErrorGrpc::Service(ServiceError::InvalidCredentials),

            tonic::Code::InvalidArgument => {
                AppErrorGrpc::Service(ServiceError::Validation(vec![status.message().to_string()]))
            }

            tonic::Code::NotFound => {
                AppErrorGrpc::Service(ServiceError::Repo(RepositoryError::NotFound))
            }

            tonic::Code::PermissionDenied => {
                AppErrorGrpc::Service(ServiceError::Forbidden(status.message().to_string()))
            }

            tonic::Code::AlreadyExists => AppErrorGrpc::Service(ServiceError::Repo(
                RepositoryError::AlreadyExists(status.message().to_string()),
            )),

            tonic::Code::FailedPrecondition | tonic::Code::Aborted => AppErrorGrpc::Service(
                ServiceError::Repo(RepositoryError::ForeignKey(status.message().to_string())),
            ),

            tonic::Code::Internal => AppErrorGrpc::Service(ServiceError::InternalServerError(
                status.message().to_string(),
            )),

            _ => {
                warn!("🌐 Unknown gRPC status conversion: {status_code} - {message}",);
                AppErrorGrpc::Unhandled(format!("gRPC error: {status_code} - {message}"))
            }
        }
    }
}
