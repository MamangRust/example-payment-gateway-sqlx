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
    async fn find_all_role(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRole>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationRole {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_id_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();

        match self.query.find_by_id(req.role_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_active(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationRoleDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_trashed(
        &self,
        request: Request<FindAllRoleRequest>,
    ) -> Result<Response<ApiResponsePaginationRoleDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllRoles {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationRoleDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_user_id(
        &self,
        request: Request<FindByIdUserRoleRequest>,
    ) -> Result<Response<ApiResponsesRole>, Status> {
        let req = request.into_inner();

        match self.query.find_by_user_id(req.user_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsesRole {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn create_role(
        &self,
        request: Request<CreateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();
        let domain_req = DomainCreateRoleRequest { name: req.name };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>,
    ) -> Result<Response<ApiResponseRole>, Status> {
        let req = request.into_inner();
        let domain_req = DomainUpdateRoleRequest {
            id: req.id,
            name: req.name,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRole {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trash(req.role_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_role(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.role_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRoleDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_role_permanent(
        &self,
        request: Request<FindByIdRoleRequest>,
    ) -> Result<Response<ApiResponseRoleDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete(req.role_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRoleDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_role(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_role_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseRoleAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseRoleAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
