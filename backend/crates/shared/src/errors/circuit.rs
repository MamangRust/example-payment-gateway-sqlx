use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum CircuitBreakerError<E> {
    #[error("circuit breaker open")]
    Open,
    #[error(transparent)]
    Inner(E),
}

impl From<CircuitBreakerError<Status>> for Status {
    fn from(e: CircuitBreakerError<Status>) -> Self {
        match e {
            CircuitBreakerError::Open => Status::unavailable("Service temporarily unavailable"),
            CircuitBreakerError::Inner(s) => s,
        }
    }
}
