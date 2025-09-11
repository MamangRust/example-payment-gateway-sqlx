use async_trait::async_trait;
use genproto::role::{
    CreateRoleRequest, FindAllRoleRequest, FindByIdRoleRequest, FindByIdUserRoleRequest,
    UpdateRoleRequest, role_service_client::RoleServiceClient,
};
use shared::{
    abstract_trait::role::http::{
        command::RoleCommandGrpcClientTrait, query::RoleQueryGrpcClientTrait,
    },
    domain::{
        requests::role::{
            CreateRoleRequest as DomainCreateRoleRequest, FindAllRoles as DomainFindAllRoles,
            UpdateRoleRequest as DomainUpdateRoleRequest,
        },
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::{AppErrorGrpc, AppErrorHttp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[async_trait]
#[allow(dead_code)]
pub trait RoleGrpcClientServiceTrait:
    RoleQueryGrpcClientTrait + RoleCommandGrpcClientTrait
{
}

#[derive(Debug)]
pub struct RoleGrpcClientService {
    client: Arc<Mutex<RoleServiceClient<Channel>>>,
}

impl RoleGrpcClientService {
    pub async fn new(client: Arc<Mutex<RoleServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RoleQueryGrpcClientTrait for RoleGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, AppErrorHttp> {
        info!(
            "fetching all roles - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllRoleRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<RoleResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} roles", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all roles failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_active(
        &self,
        request: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active roles - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllRoleRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<RoleResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active roles", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active roles failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_trashed(
        &self,
        request: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed roles - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllRoleRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<RoleResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed roles", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed roles failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, AppErrorHttp> {
        info!("fetching role by id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdRoleRequest { role_id: id });

        match client.find_by_id_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("role {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Role data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found role {id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find role {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_user_id(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, AppErrorHttp> {
        info!("fetching roles by user_id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdUserRoleRequest { user_id: id });

        match client.find_by_user_id(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<RoleResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} roles for user_id {id}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch roles for user_id {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl RoleCommandGrpcClientTrait for RoleGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: &DomainCreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, AppErrorHttp> {
        info!("creating role with name: {}", request.name);

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(CreateRoleRequest {
            name: request.name.clone(),
        });

        match client.create_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "role creation failed - data missing in gRPC response for name: {}",
                        request.name
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Role data is missing in gRPC response".into(),
                    ))
                })?;

                info!("role '{}' created successfully", request.name);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create role '{}' failed: {status:?}", request.name);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: &DomainUpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, AppErrorHttp> {
        info!(
            "updating role id: {} with name: {}",
            request.id, request.name
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(UpdateRoleRequest {
            id: request.id,
            name: request.name.clone(),
        });

        match client.update_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update role {} - data missing in gRPC response", request.id);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Role data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "role {} updated successfully to name '{}'",
                    request.id, request.name
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update role {} failed: {status:?}", request.id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, AppErrorHttp> {
        info!("trashing role id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdRoleRequest { role_id: id });

        match client.trashed_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash role {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Role data is missing in gRPC response".into(),
                    ))
                })?;

                info!("role {id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash role {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, AppErrorHttp> {
        info!("restoring role id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdRoleRequest { role_id: id });

        match client.restore_role(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore role {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Role data is missing in gRPC response".into(),
                    ))
                })?;

                info!("role {id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore role {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting role id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdRoleRequest { role_id: id });

        match client.delete_role_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("role {id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete role {id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed roles");

        let mut client = self.client.lock().await;

        match client.restore_all_role(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed roles restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all roles failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all roles");

        let mut client = self.client.lock().await;

        match client.delete_all_role_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all roles permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all roles permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl RoleGrpcClientServiceTrait for RoleGrpcClientService {}
