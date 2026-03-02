use std::sync::Arc;

use genproto::auth::{
    ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister, GetMeRequest,
    LoginRequest, RefreshTokenRequest, RegisterRequest, auth_service_server::AuthService,
};
use shared::{
    domain::requests::auth::{AuthRequest, RegisterRequest as RegisterDomainRequest},
    errors::{AppErrorGrpc, CircuitBreakerError},
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

use crate::state::AppState;

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub state: Arc<AppState>,
}

impl AuthServiceImpl {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    async fn check_rate_limit(&self) -> Result<(), Status> {
        self.state.load_monitor.record_request();

        if self.state.circuit_breaker.is_open() {
            warn!("Request rejected: circuit breaker open");
            return Err(Status::unavailable(
                "Service temporarily unavailable due to high error rate. Please try again later.",
            ));
        }

        match self.state.di_container.request_limiter.try_acquire() {
            Ok(_permit) => Ok(()),
            Err(_) => {
                warn!("Request rejected: rate limit exceeded");
                Err(Status::resource_exhausted(
                    "Too many concurrent requests. Please try again later.",
                ))
            }
        }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    #[instrument(skip(self, request), fields(method = "register_user"))]
    async fn register_user(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<ApiResponseRegister>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = RegisterDomainRequest {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email.clone(),
            password: req.password,
            confirm_password: req.confirm_password,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .auth_service
                    .register_user(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let reply = ApiResponseRegister {
                    status: api_response.status,
                    message: api_response.message.clone(),
                    data: Some(api_response.data.into()),
                };

                Ok(Response::new(reply))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("User registered successfully: {}", resp.get_ref().message);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            email = domain_req.email,
                            "register_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            email = domain_req.email,
                            error = %inner,
                            "register_user failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "login_user"))]
    async fn login_user(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<ApiResponseLogin>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = AuthRequest {
            email: req.email.clone(),
            password: req.password,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .auth_service
                    .login_user(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let reply = ApiResponseLogin {
                    status: api_response.status,
                    message: api_response.message.clone(),
                    data: Some(api_response.data.into()),
                };

                Ok(Response::new(reply))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("User login success: {}", resp.get_ref().message);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            email = domain_req.email,
                            "login_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            email = domain_req.email,
                            error = %inner,
                            "login_user failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "refresh_token"))]
    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<ApiResponseRefreshToken>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let refresh_token = req.refresh_token.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .auth_service
                    .refresh_token(&refresh_token)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let reply = ApiResponseRefreshToken {
                    status: api_response.status,
                    message: api_response.message.clone(),
                    data: Some(api_response.data.into()),
                };

                Ok(Response::new(reply))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Refresh token success: {}", resp.get_ref().message);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("refresh_token rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            error = %inner,
                            "refresh_token failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "get_me"))]
    async fn get_me(
        &self,
        request: Request<GetMeRequest>,
    ) -> Result<Response<ApiResponseGetMe>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .auth_service
                    .get_me(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let reply = ApiResponseGetMe {
                    status: api_response.status,
                    message: api_response.message.clone(),
                    data: Some(api_response.data.into()),
                };

                Ok(Response::new(reply))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("GetMe success: {}", resp.get_ref().message);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(user_id = user_id, "get_me rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            user_id = user_id,
                            error = %inner,
                            "get_me failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
}
