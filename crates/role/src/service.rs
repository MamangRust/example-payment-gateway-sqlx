use genproto::role::{
    ApiResponsePaginationRole, ApiResponsePaginationRoleDeleteAt, ApiResponseRole,
    ApiResponseRoleAll, ApiResponseRoleDelete, ApiResponseRoleDeleteAt, ApiResponsesRole,
    CreateRoleRequest, FindAllRoleRequest, FindByIdRoleRequest, FindByIdUserRoleRequest,
    UpdateRoleRequest, role_service_server::RoleService,
};
use shared::{
    abstract_trait::role::service::{command::DynRoleCommandService, query::DynRoleQueryService},
    domain::requests::role::{
        CreateRoleRequest as DomainCreateRoleRequest, FindAllRoles,
        UpdateRoleRequest as DomainUpdateRoleRequest,
    },
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct RoleServiceImpl {
    pub query: DynRoleQueryService,
    pub command: DynRoleCommandService,
}

impl RoleServiceImpl {
    pub fn new(query: DynRoleQueryService, command: DynRoleCommandService) -> Self {
        Self { query, command }
    }
}

#[tonic::async_trait]
impl RoleService for RoleServiceImpl {
    #[instrument(skip(self, request), fields(method = "find_all_role"))]
    async fn find_all_role(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRole>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_all_role request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Roles fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationRole {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch roles: page={}, page_size={}, error={:?}",
                    req.page, req.page_size, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_role"))]
    async fn find_by_id_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_id_role request for role_id={}",
            req.role_id
        );

        match self.query.find_by_id(req.role_id).await {
            Ok(api_response) => {
                info!("Role fetched successfully for role_id={}", req.role_id);
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to fetch role by ID {}: {:?}", req.role_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_active"))]
    async fn find_by_active(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_active request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Active roles fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationRoleDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch active roles: page={}, page_size={}, error={:?}",
                    req.page, req.page_size, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_trashed"))]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_trashed request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Trashed roles fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationRoleDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch trashed roles: page={}, page_size={}, error={:?}",
                    req.page, req.page_size, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_user_id"))]
    async fn find_by_user_id(
        &self,
        request: Request<FindByIdUserRoleRequest>,
    ) -> Result<Response<ApiResponsesRole>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_user_id request for user_id={}",
            req.user_id
        );

        match self.query.find_by_user_id(req.user_id).await {
            Ok(api_response) => {
                info!("Roles for user_id={} fetched successfully", req.user_id);
                let grpc_response = ApiResponsesRole {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to fetch roles for user_id={}: {:?}", req.user_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "create_role"))]
    async fn create_role(
        &self,
        request: Request<CreateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();
        info!("Received create_role request with name={}", req.name);

        let domain_req = DomainCreateRoleRequest {
            name: req.name.clone(),
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                info!("Role created successfully with name={}", req.name.clone());
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to create role with name={}: {:?}", req.name, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "update_role"))]
    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();
        info!(
            "Received update_role request for id={}, new name={}",
            req.id, req.name
        );

        let domain_req = DomainUpdateRoleRequest {
            id: req.id,
            name: req.name,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                info!("Role updated successfully for id={}", req.id);
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to update role for id={}: {:?}", req.id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_role"))]
    async fn trashed_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        let req = request.into_inner();
        info!("Received trashed_role request for role_id={}", req.role_id);

        match self.command.trash(req.role_id).await {
            Ok(api_response) => {
                info!("Role trashed successfully for role_id={}", req.role_id);
                let grpc_response = ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to trash role for role_id={}: {:?}", req.role_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_role"))]
    async fn restore_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        let req = request.into_inner();
        info!("Received restore_role request for role_id={}", req.role_id);

        match self.command.restore(req.role_id).await {
            Ok(api_response) => {
                info!("Role restored successfully for role_id={}", req.role_id);
                let grpc_response = ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to restore role for role_id={}: {:?}",
                    req.role_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_role_permanent"))]
    async fn delete_role_permanent(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDelete>, Status> {
        let req = request.into_inner();
        info!(
            "Received delete_role_permanent request for role_id={}",
            req.role_id
        );

        match self.command.delete(req.role_id).await {
            Ok(api_response) => {
                info!("Role permanently deleted for role_id={}", req.role_id);
                let grpc_response = ApiResponseRoleDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to permanently delete role for role_id={}: {:?}",
                    req.role_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_role"))]
    async fn restore_all_role(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        info!("Received restore_all_role request");

        match self.command.restore_all().await {
            Ok(api_response) => {
                info!("All trashed roles restored successfully");
                let grpc_response = ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to restore all roles: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_role_permanent"))]
    async fn delete_all_role_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        info!("Received delete_all_role_permanent request");

        match self.command.delete_all().await {
            Ok(api_response) => {
                info!("All roles permanently deleted successfully");
                let grpc_response = ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to permanently delete all roles: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
