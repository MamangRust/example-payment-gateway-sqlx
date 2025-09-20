use genproto::user::{
    ApiResponsePaginationUser, ApiResponsePaginationUserDeleteAt, ApiResponseUser,
    ApiResponseUserAll, ApiResponseUserDelete, ApiResponseUserDeleteAt, CreateUserRequest,
    FindAllUserRequest, FindByIdUserRequest, UpdateUserRequest, user_service_server::UserService,
};
use shared::{
    abstract_trait::user::service::{command::DynUserCommandService, query::DynUserQueryService},
    domain::requests::user::{
        CreateUserRequest as DomainCreateUserRequest, FindAllUserRequest as DomainFindAllRequest,
        UpdateUserRequest as DomainUserRequest,
    },
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct UserServiceImpl {
    pub query: DynUserQueryService,
    pub command: DynUserCommandService,
}

impl UserServiceImpl {
    pub fn new(query: DynUserQueryService, command: DynUserCommandService) -> Self {
        Self { query, command }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUser>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Fetching all users page={} page_size={}",
            req.page, req.page_size
        );

        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} users", api_response.data.len());
                let grpc_response = ApiResponsePaginationUser {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch users: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_id(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
        let req = request.into_inner();
        info!("🔍 Finding user by id={}", req.id);

        match self.query.find_by_id(req.id).await {
            Ok(api_response) => {
                info!("✅ User found id={}", req.id);
                let grpc_response = ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to find user id={}: {:?}", req.id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUserDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Fetching active users page={} page_size={}",
            req.page, req.page_size
        );

        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        match self.query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} active users", api_response.data.len());
                let grpc_response = ApiResponsePaginationUserDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch active users: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllUserRequest>,
    ) -> Result<Response<ApiResponsePaginationUserDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Fetching trashed users page={} page_size={}",
            req.page, req.page_size
        );

        let domain_req = DomainFindAllRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        match self.query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} trashed users", api_response.data.len());
                let grpc_response = ApiResponsePaginationUserDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed users: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
        let req = request.into_inner();
        info!("🆕 Creating user email={}", req.email);

        let domain_req = DomainCreateUserRequest {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email.clone(),
            password: req.password,
            confirm_password: req.confirm_password,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                info!("✅ User created successfully email={}", req.email);
                let grpc_response = ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to create user email={}: {:?}", req.email, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<ApiResponseUser>, Status> {
        let req = request.into_inner();
        info!("✏️ Updating user id={} email={}", req.id, req.email);

        let domain_req = DomainUserRequest {
            id: Some(req.id),
            firstname: Some(req.firstname),
            lastname: Some(req.lastname),
            email: Some(req.email.clone()),
            password: req.password,
            confirm_password: req.confirm_password,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                info!("✅ User updated id={}", req.id);
                let grpc_response = ApiResponseUser {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to update user id={}: {:?}", req.id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn trashed_user(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDeleteAt>, Status> {
        let req = request.into_inner();
        info!("🗑️ Moving user id={} to trash", req.id);

        match self.command.trashed(req.id).await {
            Ok(api_response) => {
                info!("✅ User moved to trash id={}", req.id);
                let grpc_response = ApiResponseUserDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to trash user id={}: {:?}", req.id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn restore_user(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDeleteAt>, Status> {
        let req = request.into_inner();
        info!("♻️ Restoring user id={}", req.id);

        match self.command.restore(req.id).await {
            Ok(api_response) => {
                info!("✅ User restored id={}", req.id);
                let grpc_response = ApiResponseUserDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore user id={}: {:?}", req.id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn delete_user_permanent(
        &self,
        request: Request<FindByIdUserRequest>,
    ) -> Result<Response<ApiResponseUserDelete>, Status> {
        let req = request.into_inner();
        info!("🔥 Permanently deleting user id={}", req.id);

        match self.command.delete_permanent(req.id).await {
            Ok(api_response) => {
                info!("✅ User permanently deleted id={}", req.id);
                let grpc_response = ApiResponseUserDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to permanently delete user id={}: {:?}",
                    req.id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all_user(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseUserAll>, Status> {
        info!("♻️ Restoring all users");

        match self.command.restore_all().await {
            Ok(api_response) => {
                info!("✅ All users restored");
                let grpc_response = ApiResponseUserAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore all users: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_user_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseUserAll>, Status> {
        info!("🔥 Permanently deleting ALL users");

        match self.command.delete_all().await {
            Ok(api_response) => {
                info!("✅ All users permanently deleted");
                let grpc_response = ApiResponseUserAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all users: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
