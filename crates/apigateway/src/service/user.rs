use async_trait::async_trait;
use genproto::user::{
    CreateUserRequest, FindAllUserRequest, FindByIdUserRequest, UpdateUserRequest,
    user_service_client::UserServiceClient,
};
use shared::{
    abstract_trait::user::http::{
        command::UserCommandGrpcClientTrait, query::UserQueryGrpcClientTrait,
    },
    domain::{
        requests::user::{
            CreateUserRequest as DomainCreateUserRequest,
            FindAllUserRequest as DomainFindAllUserRequest,
            UpdateUserRequest as DomainUpdateUserRequest,
        },
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::{AppErrorGrpc, AppErrorHttp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[async_trait]
pub trait UserGrpcClientTrait: UserCommandGrpcClientTrait + UserQueryGrpcClientTrait {}

#[derive(Debug)]
pub struct UserGrpcClientService {
    client: Arc<Mutex<UserServiceClient<Channel>>>,
}

impl UserGrpcClientService {
    pub async fn new(client: Arc<Mutex<UserServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserQueryGrpcClientTrait for UserGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, AppErrorHttp> {
        info!(
            "fetching all users - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllUserRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<UserResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} users", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all users failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, AppErrorHttp> {
        info!("fetching user by id: {user_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdUserRequest { id: user_id });

        match client.find_by_id(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("user {user_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "User data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found user {user_id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find user {user_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active users - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllUserRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<UserResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active users", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active users failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed users - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllUserRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<UserResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed users", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed users failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl UserCommandGrpcClientTrait for UserGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp> {
        info!(
            "creating user: {} {} - email: {}",
            req.firstname, req.lastname, req.email
        );

        let mut client = self.client.lock().await;

        let grpc_req = CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            email: req.email.clone(),
            password: req.password.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        match client.create(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "user creation failed - data missing in gRPC response for email: {}",
                        req.email
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "User data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "user {} {} created successfully",
                    req.firstname, req.lastname
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!(
                    "create user {} {} (email: {}) failed: {status:?}",
                    req.firstname, req.lastname, req.email
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp> {
        info!(
            "updating user id: {} - firstname: {:?}, lastname: {:?}, email: {:?}",
            req.id, req.firstname, req.lastname, req.email
        );

        let mut client = self.client.lock().await;

        let grpc_req = UpdateUserRequest {
            id: req.id,
            firstname: req.firstname.clone().unwrap_or_default(),
            lastname: req.lastname.clone().unwrap_or_default(),
            email: req.email.clone().unwrap_or_default(),
            password: req.password.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        match client.update(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update user {} - data missing in gRPC response", req.id);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "User data is missing in gRPC response".into(),
                    ))
                })?;

                info!("user {} updated successfully", req.id);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update user {} failed: {status:?}", req.id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, AppErrorHttp> {
        info!("trashing user id: {user_id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdUserRequest { id: user_id };

        match client.trashed_user(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash user {user_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "User data is missing in gRPC response".into(),
                    ))
                })?;

                info!("user {user_id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash user {user_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, AppErrorHttp> {
        info!("restoring user id: {user_id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdUserRequest { id: user_id };

        match client.restore_user(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore user {user_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "User data is missing in gRPC response".into(),
                    ))
                })?;

                info!("user {user_id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore user {user_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting user id: {user_id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdUserRequest { id: user_id };

        match client.delete_user_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("user {user_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete user {user_id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed users");

        let mut client = self.client.lock().await;

        match client.restore_all_user(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed users restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all users failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all users");

        let mut client = self.client.lock().await;

        match client.delete_all_user_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all users permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all users permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl UserGrpcClientTrait for UserGrpcClientService {}
