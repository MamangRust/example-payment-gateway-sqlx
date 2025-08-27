use crate::errors::{repository::RepositoryError, service::ServiceError};
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum AppErrorGrpc {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Unhandled: {0}")]
    Unhandled(String),
}

impl From<AppErrorGrpc> for Status {
    fn from(err: AppErrorGrpc) -> Self {
        match err {
            AppErrorGrpc::Service(service_err) => match service_err {
                ServiceError::InvalidCredentials => Status::unauthenticated("Invalid credentials"),

                ServiceError::Validation(errors) => {
                    Status::invalid_argument(format!("Validation failed: {errors:#?}"))
                }

                ServiceError::Repo(repo_err) => match repo_err {
                    RepositoryError::NotFound => Status::not_found("Not found"),
                    RepositoryError::Conflict(msg) => Status::already_exists(&msg),
                    RepositoryError::AlreadyExists(msg) => Status::already_exists(&msg),
                    RepositoryError::ForeignKey(msg) => {
                        Status::failed_precondition(format!("Foreign key constraint: {msg}"))
                    }
                    RepositoryError::Sqlx(_) => Status::internal("Database error"),
                    RepositoryError::Custom(msg) => Status::internal(&msg),
                },

                ServiceError::Bcrypt(err) => Status::internal(format!("Bcrypt error: {err}")),

                ServiceError::Jwt(err) => Status::unauthenticated(format!("JWT error: {err}")),

                ServiceError::TokenExpired => Status::unauthenticated("Token has expired"),

                ServiceError::InvalidTokenType => Status::unauthenticated("Invalid token type"),

                ServiceError::Internal(msg) => Status::internal(msg),

                ServiceError::Custom(msg) => Status::internal(msg),
            },

            AppErrorGrpc::Unhandled(msg) => Status::internal(format!("Unhandled error: {msg}")),
        }
    }
}

impl From<Status> for AppErrorGrpc {
    fn from(status: Status) -> Self {
        match status.code() {
            tonic::Code::Unauthenticated => AppErrorGrpc::Service(ServiceError::InvalidCredentials),

            tonic::Code::InvalidArgument => {
                AppErrorGrpc::Service(ServiceError::Validation(vec![status.message().to_string()]))
            }

            tonic::Code::NotFound => {
                AppErrorGrpc::Service(ServiceError::Repo(RepositoryError::NotFound))
            }

            tonic::Code::AlreadyExists => AppErrorGrpc::Service(ServiceError::Repo(
                RepositoryError::AlreadyExists(status.message().to_string()),
            )),

            tonic::Code::FailedPrecondition | tonic::Code::Aborted => AppErrorGrpc::Service(
                ServiceError::Repo(RepositoryError::ForeignKey(status.message().to_string())),
            ),

            tonic::Code::Internal => {
                AppErrorGrpc::Service(ServiceError::Internal(status.message().to_string()))
            }

            _ => AppErrorGrpc::Unhandled(format!(
                "gRPC error: {} - {}",
                status.code(),
                status.message()
            )),
        }
    }
}
