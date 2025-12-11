use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::role::{
    CreateRoleRequest, FindAllRoleRequest, FindByIdRoleRequest, FindByIdUserRoleRequest,
    UpdateRoleRequest, role_service_client::RoleServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::role::http::{
        RoleCommandGrpcClientTrait, RoleGrpcClientServiceTrait, RoleQueryGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::role::{
            CreateRoleRequest as DomainCreateRoleRequest, FindAllRoles as DomainFindAllRoles,
            UpdateRoleRequest as DomainUpdateRoleRequest,
        },
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct RoleGrpcClientService {
    client: RoleServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl RoleGrpcClientService {
    pub fn new(client: RoleServiceClient<Channel>, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("role-client-service")
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
impl RoleGrpcClientServiceTrait for RoleGrpcClientService {}

#[async_trait]
impl RoleQueryGrpcClientTrait for RoleGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "Retrieving all role (page: {page}, size: {page_size} search: {})",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut request = Request::new(FindAllRoleRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "role:find_all:page:{page}:size:{page_size}:search:{}",
            req.search.clone(),
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} roles in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let response = match self.client.clone().find_all_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched roles")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch roles")
                    .await;

                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let data: Vec<RoleResponse> = inner.data.into_iter().map(Into::into).collect();

        let pagination = inner.pagination.map(Into::into).unwrap_or_default();

        let api_response = ApiResponsePagination {
            data,
            pagination,
            message: inner.message,
            status: inner.status,
        };

        let role_len = api_response.data.len();

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
            .await;

        info!("Successfully fetched {role_len} Roles");

        Ok(api_response)
    }
    #[instrument(skip(self, req), level = "info")]
    async fn find_active(
        &self,
        req: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "Retrieving all active role (page: {page}, size: {page_size} search: {})",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut request = Request::new(FindAllRoleRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "role:find_active:page:{page}:size:{page_size}:search:{}",
            req.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active roles in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let response = match self.client.clone().find_by_active(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched roles")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch roles")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let data: Vec<RoleResponseDeleteAt> = inner.data.into_iter().map(Into::into).collect();

        let pagination = inner.pagination.map(Into::into).unwrap_or_default();

        let api_response = ApiResponsePagination {
            data,
            pagination,
            message: inner.message,
            status: inner.status,
        };

        let roles_len = api_response.data.len();

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
            .await;

        info!("Successfully fetched {roles_len} active Roles");
        Ok(api_response)
    }
    #[instrument(skip(self, req), level = "info")]
    async fn find_trashed(
        &self,
        req: &DomainFindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "Retrieving all trashed role (page: {page}, size: {page_size} search: {:?})",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut request = Request::new(FindAllRoleRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "role:find_trashed:page:{page}:size:{page_size}:search:{:?}",
            req.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed roles in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let response = match self.client.clone().find_by_trashed(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched roles")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch roles")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let data: Vec<RoleResponseDeleteAt> = inner.data.into_iter().map(Into::into).collect();

        let pagination = inner.pagination.map(Into::into).unwrap_or_default();

        let api_response = ApiResponsePagination {
            data,
            pagination,
            message: inner.message,
            status: inner.status,
        };

        let roles_len = api_response.data.len();

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
            .await;

        info!("Successfully fetched {roles_len} trashed Roles");

        Ok(api_response)
    }
    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, HttpError> {
        info!("Retrieving Role: {}", id);

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindByIdRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("role.id", id.to_string()),
            ],
        );

        let mut request = Request::new(FindByIdRoleRequest { role_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("role:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<RoleResponse>>(&cache_key)
            .await
        {
            info!("✅ Found role in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Role retrieved from cache")
                .await;
            return Ok(cache);
        }

        let response = match self.client.clone().find_by_id_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched Role")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let role_data = inner
            .data
            .ok_or_else(|| HttpError::Internal("Role data is missing in gRPC response".into()))?;

        let data: RoleResponse = role_data.into();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data,
        };
        let role_name = api_response.data.clone().name;

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
            .await;

        info!("Successfully fetched Role: {role_name}");

        Ok(api_response)
    }
    #[instrument(skip(self), level = "info")]
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, HttpError> {
        info!("Fetching Roles by user_id: {}", user_id);

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindByIdUserRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_by_user_id"),
                KeyValue::new("user.id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(FindByIdUserRoleRequest { user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("role:find_by_user_id:user_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<RoleResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found roles for user in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Roles for user retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let response = match self.client.clone().find_by_user_id(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched roles")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch roles")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let data: Vec<RoleResponse> = inner.data.into_iter().map(RoleResponse::from).collect();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
            .await;

        info!(
            "Successfully fetched {} roles for user_id {}",
            api_response.data.len(),
            user_id
        );
        Ok(api_response)
    }
}

#[async_trait]
impl RoleCommandGrpcClientTrait for RoleGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, HttpError> {
        info!("Creating new Role: {}", req.name.clone());

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "create"),
                KeyValue::new("role.name", req.name.clone()),
            ],
        );

        let mut request = Request::new(CreateRoleRequest {
            name: req.name.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().create_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully created Role")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let role_data = inner
            .data
            .ok_or_else(|| HttpError::Internal("Role data is missing in gRPC response".into()))?;

        let data: RoleResponse = role_data.into();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data,
        };

        let cache_keys = vec![
            "role:find_all:*",
            "role:find_by_active:*",
            "role_find_by_trashed",
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(key).await;
        }

        let cache_key = format!("role:find_by_id:{}", api_response.data.id);

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
            .await;

        info!("Role {} created successfully", req.name);
        Ok(api_response)
    }
    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, HttpError> {
        let id = req
            .id
            .ok_or_else(|| HttpError::Internal("id is required".to_string()))?;

        info!("Updating Role: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "update"),
                KeyValue::new("role.id", id.to_string()),
                KeyValue::new("role.name", req.name.clone()),
            ],
        );

        let mut request = Request::new(UpdateRoleRequest {
            id,
            name: req.name.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().update_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully updated Role")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let role_data = inner
            .data
            .ok_or_else(|| HttpError::Internal("Role data is missing in gRPC response".into()))?;

        let data: RoleResponse = role_data.into();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data,
        };
        let role_name = api_response.data.clone().name;
        let id = api_response.data.clone().id;

        let cache_keys = vec![
            format!("role:find_by_id:{id}"),
            "role:find_all:*".to_string(),
            "role:find_by_active:*".to_string(),
            "role_find_by_trashed".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        let cache_key = format!("role:find_by_id:{id}");

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
            .await;

        info!("Role {role_name} updated successfully");

        Ok(api_response)
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, HttpError> {
        info!("Soft deleting Role: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("role_id", id.to_string()),
            ],
        );

        let mut request = Request::new(FindByIdRoleRequest { role_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().trashed_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully soft deleted Role",
                )
                .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to soft delete Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let role_data = inner
            .data
            .ok_or_else(|| HttpError::Internal("Role data is missing in gRPC response".into()))?;

        let domain_role: RoleResponseDeleteAt = role_data.into();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_role,
        };

        let cache_keys = vec![
            format!("role:find_by_id:id:{}", id),
            format!("role:find_by_name:name:{}", api_response.data.name),
            "role:find_all:*".to_string(),
            "role:find_active:*".to_string(),
            "role:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        info!("Role {} soft deleted successfully", id);
        Ok(api_response)
    }
    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, HttpError> {
        info!("Restoring Role: {}", id);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("role_id", id.to_string()),
            ],
        );

        let mut request = Request::new(FindByIdRoleRequest { role_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().restore_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully restored Role")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let role_data = inner
            .data
            .ok_or_else(|| HttpError::Internal("Role data is missing in gRPC response".into()))?;

        let data: RoleResponseDeleteAt = role_data.into();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data,
        };

        let cache_keys = vec![
            format!("role:find_by_id:id:{id}"),
            format!("role:find_by_name:name:{}", api_response.data.name),
            "role:find_all:*".to_string(),
            "role:find_active:*".to_string(),
            "role:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        info!("Role {id} restored successfully");
        Ok(api_response)
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("Permanently deleting Role: {id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("role_id", id.to_string()),
            ],
        );

        let mut request = Request::new(FindByIdRoleRequest { role_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().delete_role_permanent(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully deleted Role")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to delete Role")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data: true,
        };

        info!("Role {} permanently deleted", id);
        Ok(api_response)
    }
    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("Restoring all trashed Roles");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "restore"),
            ],
        );

        let mut request = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().restore_all_role(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "All Roles restored")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore all Roles")
                    .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data: true,
        };

        let cache_keys = vec![
            "role:find_trashed:*",
            "role:find_active:*",
            "role:find_all:*",
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(key).await;
        }

        info!("All Roles restored successfully");
        Ok(api_response)
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("Permanently deleting all trashed Roles");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllRole",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "delete"),
            ],
        );

        let mut request = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().delete_all_role_permanent(request).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "All trashed Roles deleted")
                    .await;
                response
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all trashed Roles",
                )
                .await;
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner = response.into_inner();

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data: true,
        };

        let cache_keys = vec![
            "role:find_trashed:*",
            "role:find_active:*",
            "role:find_all:*",
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(key).await;
        }

        info!("All trashed Roles permanently deleted");
        Ok(api_response)
    }
}
