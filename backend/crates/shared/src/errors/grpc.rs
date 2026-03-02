use crate::errors::{CircuitBreakerError, repository::RepositoryError, service::ServiceError};
use opentelemetry::Context;
use opentelemetry::trace::{TraceContextExt, TraceId};
use thiserror::Error;
use tonic::{Code, Status, metadata::MetadataMap};
use tracing::{error, warn};

#[derive(Debug, Error)]
pub enum AppErrorGrpc {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Circuit breaker is open - service temporarily unavailable")]
    CircuitBreakerOpen,
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
            AppErrorGrpc::CircuitBreakerOpen => warn!("🔌 {}", self),
            AppErrorGrpc::Unhandled(_) => error!("💥 {}", self),
        }
    }
}

impl From<AppErrorGrpc> for Status {
    fn from(err: AppErrorGrpc) -> Self {
        err.log();

        let binding = Context::current();
        let span = binding.span();
        let span_ctx = span.span_context();
        let trace_id = span_ctx.trace_id();

        let mut metadata = MetadataMap::new();

        if trace_id != TraceId::INVALID {
            if let Ok(trace_val) = trace_id.to_string().parse() {
                metadata.insert("x-trace-id", trace_val);
            }
        }

        let (code, message) = match err {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => {
                    (Code::Unauthenticated, "🔐 Invalid credentials".into())
                }
                ServiceError::Validation(errors) => (
                    Code::InvalidArgument,
                    format!("📝 Validation failed: {errors:#?}"),
                ),
                ServiceError::Forbidden(msg) => (Code::PermissionDenied, msg),
                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => (Code::NotFound, "🔍 Resource not found".into()),
                    RepositoryError::Conflict(msg) => {
                        (Code::AlreadyExists, format!("⚡ Conflict: {msg}"))
                    }
                    RepositoryError::AlreadyExists(msg) => {
                        (Code::AlreadyExists, format!("📦 Already exists: {msg}"))
                    }
                    RepositoryError::ForeignKey(msg) => (
                        Code::FailedPrecondition,
                        format!("🔗 Foreign key constraint: {msg}"),
                    ),
                    RepositoryError::Sqlx(err) => {
                        error!("💾 Database SQLx error: {err:?}");
                        (Code::Internal, "💾 Database operation failed".into())
                    }
                    RepositoryError::Custom(msg) => {
                        warn!("⚙️ Custom repository error: {msg}");
                        (Code::Internal, format!("⚙️ {msg}"))
                    }
                },
                ServiceError::Bcrypt(err) => {
                    error!("🔒 Bcrypt error: {err:?}");
                    (Code::Internal, "🔒 Password processing error".into())
                }
                ServiceError::Jwt(err) => {
                    warn!("🎫 JWT error: {err:?}");
                    (Code::Unauthenticated, format!("🎫 Token error: {err:?}"))
                }
                ServiceError::TokenExpired => {
                    (Code::Unauthenticated, "⏰ Token has expired".into())
                }
                ServiceError::InvalidTokenType => {
                    (Code::Unauthenticated, "🎫 Invalid token type".into())
                }
                ServiceError::InternalServerError(msg) => {
                    error!("🔥 Internal server error: {msg}");
                    (Code::Internal, format!("🔥 {msg}"))
                }
                ServiceError::Custom(msg) => {
                    warn!("⚙️ Custom service error: {msg}");
                    (Code::Internal, format!("⚙️ {msg}"))
                }
                ServiceError::NotFound(msg) => {
                    warn!("🔍 Not found: {msg}");
                    (Code::NotFound, format!("🔍 {msg}"))
                }
            },
            AppErrorGrpc::CircuitBreakerOpen => (
                Code::Unavailable,
                "🔌 Service temporarily unavailable - circuit breaker is open".into(),
            ),
            AppErrorGrpc::Unhandled(msg) => {
                error!("💥 Unhandled application error: {msg}");
                (Code::Internal, format!("💥 Unexpected error: {msg}"))
            }
        };

        Status::with_metadata(code, message, metadata)
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

            tonic::Code::Unavailable => AppErrorGrpc::CircuitBreakerOpen,

            _ => {
                warn!("🌐 Unknown gRPC status conversion: {status_code} - {message}");
                AppErrorGrpc::Unhandled(format!("gRPC error: {status_code} - {message}"))
            }
        }
    }
}

impl<E> From<CircuitBreakerError<E>> for AppErrorGrpc
where
    E: Into<AppErrorGrpc>,
{
    fn from(err: CircuitBreakerError<E>) -> Self {
        match err {
            CircuitBreakerError::Open => AppErrorGrpc::CircuitBreakerOpen,
            CircuitBreakerError::Inner(e) => e.into(),
        }
    }
}
