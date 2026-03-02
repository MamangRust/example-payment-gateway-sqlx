use crate::state::AppState;
use genproto::role::{
    ApiResponsePaginationRole, ApiResponsePaginationRoleDeleteAt, ApiResponseRole,
    ApiResponseRoleAll, ApiResponseRoleDelete, ApiResponseRoleDeleteAt, ApiResponsesRole,
    CreateRoleRequest, FindAllRoleRequest, FindByIdRoleRequest, FindByIdUserRoleRequest,
    UpdateRoleRequest, role_service_server::RoleService,
};
use shared::{
    domain::requests::role::{
        CreateRoleRequest as DomainCreateRoleRequest, FindAllRoles,
        UpdateRoleRequest as DomainUpdateRoleRequest,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct RoleServiceImpl {
    pub state: Arc<AppState>,
}

impl RoleServiceImpl {
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
impl RoleService for RoleServiceImpl {
    #[instrument(skip(self, request), fields(method = "find_all_role"))]
    async fn find_all_role(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRole>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationRole {
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
                    "find_all_role success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_role rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_role failed"
                        );
                    }
                }

                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_role"))]
    async fn find_by_id_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let role_id = req.role_id;
        info!("Received find_by_id_role request for role_id={}", role_id);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_query
                    .find_by_id(role_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role fetched successfully for role_id={}", role_id);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            role_id = role_id,
                            "find_by_id_role rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            role_id = role_id,
                            error = %inner,
                            "find_by_id_role failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_active"))]
    async fn find_by_active(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };
        info!(
            "Received find_by_active request: page={}, page_size={}, search={:?}",
            domain_req.page, domain_req.page_size, domain_req.search
        );

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_query
                    .find_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationRoleDeleteAt {
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
                    "Active roles fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_trashed"))]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };
        info!(
            "Received find_by_trashed request: page={}, page_size={}, search={:?}",
            domain_req.page, domain_req.page_size, domain_req.search
        );

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_query
                    .find_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationRoleDeleteAt {
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
                    "Trashed roles fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_user_id"))]
    async fn find_by_user_id(
        &self,
        request: Request<FindByIdUserRoleRequest>,
    ) -> Result<Response<ApiResponsesRole>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.user_id;
        info!("Received find_by_user_id request for user_id={}", user_id);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_query
                    .find_by_user_id(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsesRole {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Roles for user_id={} fetched successfully", user_id);
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "find_by_user_id rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            user_id = user_id,
                            error = %inner,
                            "find_by_user_id failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(method = "create_role"))]
    async fn create_role(
        &self,
        request: Request<CreateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        info!("Received create_role request");

        let domain_req = DomainCreateRoleRequest {
            name: req.name.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role created successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(role_name = %req.name, "create_role rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(role_name = %req.name, error = %inner, "create_role failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "update_role"))]
    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        info!("Received update_role request");

        let domain_req = DomainUpdateRoleRequest {
            id: Some(req.id),
            name: req.name.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role updated successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("update_role rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "update_role failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_role"))]
    async fn trashed_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        info!("Received trashed_role request");

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .trash(req.role_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role trashed successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("trashed_role rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "trashed_role failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_role"))]
    async fn restore_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        info!("Received restore_role request");

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .restore(req.role_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role restored successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_role rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_role failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_role_permanent"))]
    async fn delete_role_permanent(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        info!("Received delete_role_permanent request");

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .delete(req.role_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRoleDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("Role permanently deleted successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_role_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_role_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_role"))]
    async fn restore_all_role(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        self.check_rate_limit().await?;

        info!("Received restore_all_role request");

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("All trashed roles restored successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_role rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_role failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_role_permanent"))]
    async fn delete_all_role_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        self.check_rate_limit().await?;

        info!("Received delete_all_role_permanent request");

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .role_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("All roles permanently deleted successfully");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_role_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_role_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
