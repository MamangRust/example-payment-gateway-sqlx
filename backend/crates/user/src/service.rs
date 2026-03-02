use crate::state::AppState;
use genproto::user::{
    ApiResponsePaginationUser, ApiResponsePaginationUserDeleteAt, ApiResponseUser,
    ApiResponseUserAll, ApiResponseUserDelete, ApiResponseUserDeleteAt, CreateUserRequest,
    FindAllUserRequest, FindByIdUserRequest, UpdateUserRequest, user_service_server::UserService,
};
use shared::{
    domain::requests::user::{
        CreateUserRequest as DomainCreateUserRequest, FindAllUserRequest as DomainFindAllRequest,
        UpdateUserRequest as DomainUserRequest,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct UserServiceImpl {
    pub state: Arc<AppState>,
}

impl UserServiceImpl {
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
impl UserService for UserServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_user",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUser>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .user_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationUser {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_all_user success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_user failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_user", user_id = request.get_ref().id), level = "info")]
    async fn find_by_id(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
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
                    .user_query
                    .find_by_id(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "find_by_id_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "find_by_id_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "find_by_id_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_active_user",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUserDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .user_query
                    .find_by_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationUserDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_by_active_user success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active_user failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_trashed_user",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUserDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .user_query
                    .find_by_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationUserDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_by_trashed_user success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed_user failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(method = "create_user", email = %request.get_ref().email), level = "info")]
    async fn create(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let email = req.email.clone();

        let domain_req = DomainCreateUserRequest {
            firstname: req.firstname,
            lastname: req.lastname,
            email: email.clone(),
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
                    .user_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(email = email, "create_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(email = email, "create_user rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(email = email, error = %inner, "create_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_user",
        user_id = request.get_ref().id,
        email = %request.get_ref().email
    ), level = "info")]
    async fn update(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.id;
        let email = req.email.clone();

        let domain_req = DomainUserRequest {
            id: Some(req.id),
            firstname: Some(req.firstname),
            lastname: Some(req.lastname),
            email: Some(email.clone()),
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
                    .user_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "update_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "update_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "update_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_user", user_id = request.get_ref().id), level = "info")]
    async fn trashed_user(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDeleteAt>, Status> {
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
                    .user_command
                    .trashed(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUserDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "trashed_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "trashed_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "trashed_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_user", user_id = request.get_ref().id), level = "info")]
    async fn restore_user(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDeleteAt>, Status> {
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
                    .user_command
                    .restore(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUserDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "restore_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "restore_user rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "restore_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_user_permanent", user_id = request.get_ref().id), level = "info")]
    async fn delete_user_permanent(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDelete>, Status> {
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
                    .user_command
                    .delete_permanent(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUserDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "delete_user_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "delete_user_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "delete_user_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "restore_all_user"),
        level = "info"
    )]
    async fn restore_all_user(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseUserAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .user_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUserAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_user success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_user rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_user failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "delete_all_user_permanent"),
        level = "info"
    )]
    async fn delete_all_user_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseUserAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .user_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseUserAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_user_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_user_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_user_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
