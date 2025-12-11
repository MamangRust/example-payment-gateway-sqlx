use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::user::{
    CreateUserRequest, FindAllUserRequest, FindByIdUserRequest, UpdateUserRequest,
    user_service_client::UserServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::user::http::{
        UserCommandGrpcClientTrait, UserGrpcClientServiceTrait, UserQueryGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::user::{
            CreateUserRequest as DomainCreateUserRequest,
            FindAllUserRequest as DomainFindAllUserRequest,
            UpdateUserRequest as DomainUpdateUserRequest,
        },
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct UserGrpcClientService {
    client: UserServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl UserGrpcClientService {
    pub fn new(client: UserServiceClient<Channel>, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("user-client-service")
    }
    fn inject_trace_context<T>(&self, cx: &Context, request: &mut Request<T>) {
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(cx, &mut MetadataInjector(request.metadata_mut()))
        });
    }

    fn start_tracing(&self, operation_name: &str, attributes: Vec<KeyValue>) -> TracingContext {
        let start_time = Instant::now();
        let tracer = self.get_tracer();
        let mut span = tracer
            .span_builder(operation_name.to_string())
            .with_kind(SpanKind::Server)
            .with_attributes(attributes)
            .start(&tracer);

        info!("Starting operation: {operation_name}");

        span.add_event(
            "Operation started",
            vec![
                KeyValue::new("operation", operation_name.to_string()),
                KeyValue::new("timestamp", start_time.elapsed().as_secs_f64().to_string()),
            ],
        );

        let cx = Context::current_with_span(span);
        TracingContext { cx, start_time }
    }

    async fn complete_tracing_success(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, true, message)
            .await;
    }

    async fn complete_tracing_error(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        error_message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, false, error_message)
            .await;
    }

    async fn complete_tracing_internal(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        is_success: bool,
        message: &str,
    ) {
        let status_str = if is_success { "SUCCESS" } else { "ERROR" };
        let status = if is_success {
            StatusUtils::Success
        } else {
            StatusUtils::Error
        };
        let elapsed = tracing_ctx.start_time.elapsed().as_secs_f64();

        tracing_ctx.cx.span().add_event(
            "Operation completed",
            vec![
                KeyValue::new("status", status_str),
                KeyValue::new("duration_secs", elapsed.to_string()),
                KeyValue::new("message", message.to_string()),
            ],
        );

        if is_success {
            info!("Operation completed successfully: {message}");
        } else {
            error!("Operation failed: {message}");
        }

        self.metrics.record(method, status, elapsed);

        tracing_ctx.cx.span().end();
    }
}

#[async_trait]
impl UserGrpcClientServiceTrait for UserGrpcClientService {}

#[async_trait]
impl UserQueryGrpcClientTrait for UserGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching all users - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllUserRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "user:find_all:page:{page}:size:{page_size}:search:{}",
            req.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} users in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_all(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched users")
                    .await;

                let inner = response.into_inner();
                let data: Vec<UserResponse> = inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} users", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch users")
                    .await;
                error!("fetch all users failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, HttpError> {
        info!("fetching user by id: {user_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindUserById",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdUserRequest { id: user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("user:find_by_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<UserResponse>>(&cache_key)
            .await
        {
            info!("✅ Found user with ID {user_id} in cache");
            self.complete_tracing_success(&tracing_ctx, method, "User retrieved from cache")
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_id(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched user by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("user {user_id} - data missing in gRPC response");
                    HttpError::Internal("User data is missing in gRPC response".into())
                })?;

                let user_response = data.into();

                let api_response = ApiResponse {
                    data: user_response,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found user {user_id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch user by id")
                    .await;
                error!("find user {user_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching active users - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllUserRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "user:find_by_active:page:{page}:size:{page_size}:search:{:?}",
            req.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active users in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_active(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active users",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<UserResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} active users", api_response.data.len());
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch active users")
                    .await;
                error!("fetch active users failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching trashed users - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllUserRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "user:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            req.search
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed users in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_trashed(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed users",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<UserResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} trashed users", api_response.data.len());
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch trashed users")
                    .await;
                error!("fetch trashed users failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
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
    ) -> Result<ApiResponse<UserResponse>, HttpError> {
        info!(
            "creating user: {} {} - email: {}",
            req.firstname, req.lastname, req.email
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "create"),
                KeyValue::new("user_email", req.email.clone()),
            ],
        );

        let mut grpc_req = Request::new(CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            email: req.email.clone(),
            password: req.password.clone(),
            confirm_password: req.confirm_password.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully created user")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "user creation failed - data missing in gRPC response for email: {}",
                        req.email
                    );
                    HttpError::Internal("User data is missing in gRPC response".into())
                })?;

                let data: UserResponse = data.into();

                let api_response = ApiResponse {
                    data,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    "user:find_all:*",
                    "user:find_by_active:*",
                    "user:find_by_trashed",
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                let cache_key = format!("user:find_by_id:{}", api_response.data.id);

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "user {} {} created successfully",
                    req.firstname, req.lastname
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create user")
                    .await;
                error!(
                    "create user {} {} (email: {}) failed: {status:?}",
                    req.firstname, req.lastname, req.email
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, HttpError> {
        let user_id = req
            .id
            .ok_or_else(|| HttpError::Internal("user id is required".to_string()))?;

        info!(
            "updating user id: {user_id} - firstname: {:?}, lastname: {:?}, email: {:?}",
            req.firstname, req.lastname, req.email
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "update"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(UpdateUserRequest {
            id: user_id,
            firstname: req.firstname.clone().unwrap_or_default(),
            lastname: req.lastname.clone().unwrap_or_default(),
            email: req.email.clone().unwrap_or_default(),
            password: req.password.clone(),
            confirm_password: req.confirm_password.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully updated user")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update user {user_id} - data missing in gRPC response");
                    HttpError::Internal("User data is missing in gRPC response".into())
                })?;

                let data: UserResponse = data.into();

                let api_response = ApiResponse {
                    data,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    "user:find_all:*",
                    "user:find_by_active:*",
                    "user:find_by_trashed",
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                let cache_key = format!("user:find_by_id:{}", api_response.data.id);

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("user {user_id} updated successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update user")
                    .await;
                error!("update user {user_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(&self, user_id: i32) -> Result<ApiResponse<UserResponseDeleteAt>, HttpError> {
        info!("trashing user id: {user_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdUserRequest { id: user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_user(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully trashed user")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash user {user_id} - data missing in gRPC response");
                    HttpError::Internal("User data is missing in gRPC response".into())
                })?;

                let data: UserResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("user:find_by_id:id:{}", user_id),
                    "user:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("user {user_id} trashed successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash user")
                    .await;
                error!("trash user {user_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, user_id: i32) -> Result<ApiResponse<UserResponseDeleteAt>, HttpError> {
        info!("restoring user id: {user_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreUser",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdUserRequest { id: user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_user(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully restored user")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore user {user_id} - data missing in gRPC response");
                    HttpError::Internal("User data is missing in gRPC response".into())
                })?;

                let data: UserResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("user:find_by_id:id:{}", user_id),
                    "user:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("user {user_id} restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore user")
                    .await;
                error!("restore user {user_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting user id: {user_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteUserPermanent",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdUserRequest { id: user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().delete_user_permanent(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted user permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("user:find_by_id:id:{user_id}"),
                    "user:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("user {user_id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete user permanently",
                )
                .await;
                error!("delete user {user_id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed users");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllUsers",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_user(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all trashed users",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "user:find_by_id:id:*".to_string(),
                    "user:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all trashed users restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all trashed users",
                )
                .await;
                error!("restore all users failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all users");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllUsersPermanent",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_user_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted all users permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "user:find_by_id:id:*".to_string(),
                    "user:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all users permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all users permanently",
                )
                .await;
                error!("delete all users permanently failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
