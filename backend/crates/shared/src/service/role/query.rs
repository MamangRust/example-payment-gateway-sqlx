use crate::{
    abstract_trait::role::{
        repository::query::DynRoleQueryRepository, service::query::RoleQueryServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::role::FindAllRoles,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, RoleResponse, RoleResponseDeleteAt,
        },
    },
    errors::ServiceError,
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};

pub struct RoleQueryService {
    pub query: DynRoleQueryRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl RoleQueryService {
    pub fn new(query: DynRoleQueryRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            query,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl RoleQueryServiceTrait for RoleQueryService {
    async fn find_all(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all roles | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_all_roles",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(request.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "role:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} roles in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (roles, total_items) = match self.query.find_all(request).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} roles", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all roles: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch all roles: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponse> = roles.into_iter().map(RoleResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Roles retrieved successfully".to_string(),
            data: role_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Found {} roles (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_active(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching active roles | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_active_roles",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(request.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "role:find_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active roles in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (roles, total_items) = match self.query.find_active(request).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active roles", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active roles: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch active roles: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponseDeleteAt> =
            roles.into_iter().map(RoleResponseDeleteAt::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active roles retrieved successfully".to_string(),
            data: role_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Found {} active roles (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_trashed(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️  Searching trashed roles | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_trashed_roles",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(request.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "role:find_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<RoleResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed roles in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (roles, total_items) = match self.query.find_trashed(request).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed roles", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed roles: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch trashed roles: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponseDeleteAt> =
            roles.into_iter().map(RoleResponseDeleteAt::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed roles retrieved successfully".to_string(),
            data: role_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };
        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "🗑️  Found {} trashed roles (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        info!("🔍 Finding role by ID: {id}");

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_role_by_id",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("role:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<RoleResponse>>(&cache_key)
            .await
        {
            info!("✅ Found role in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Role retrieved from cache")
                .await;
            return Ok(cache);
        }

        let role = match self.query.find_by_id(id).await {
            Ok(Some(role)) => {
                info!("✅ Found role with ID: {id}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Role retrieved successfully")
                    .await;
                role
            }
            Ok(None) => {
                error!("❌ Role with ID {id} not found");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Role not found")
                    .await;
                return Err(ServiceError::NotFound(format!(
                    "Role with ID {id} not found"
                )));
            }
            Err(e) => {
                error!("❌ Database error while finding role by ID {id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Role retrieved successfully".to_string(),
            data: RoleResponse::from(role),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, ServiceError> {
        info!("🔍 Finding roles for user ID: {user_id}");

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_roles_by_user_id",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_by_user_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("role:find_by_user_id:user_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<RoleResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found roles for user in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Roles for user retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let roles = match self.query.find_by_user_id(user_id).await {
            Ok(roles) => {
                info!("✅ Found {} roles for user ID: {user_id}", roles.len());
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Roles for user retrieved successfully",
                    )
                    .await;
                roles
            }
            Err(e) => {
                error!("❌ Failed to fetch roles for user ID {user_id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch roles for user: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let role_responses: Vec<RoleResponse> = roles.into_iter().map(RoleResponse::from).collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: "User roles retrieved successfully".to_string(),
            data: role_responses,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_name(&self, name: String) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        info!("🔍 Finding role by name: {name}");

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_role_by_name",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "find_by_name"),
                KeyValue::new("name", name.clone()),
            ],
        );

        let mut request = Request::new(name.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("role:find_by_name:name:{}", name);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<RoleResponse>>(&cache_key)
            .await
        {
            info!("✅ Found role by name in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Role by name retrieved from cache")
                .await;
            return Ok(cache);
        }

        let role = match self.query.find_by_name(&name).await {
            Ok(Some(role)) => {
                info!("✅ Found role with name: {name}");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Role by name retrieved successfully",
                    )
                    .await;
                role
            }
            Ok(None) => {
                error!("❌ Role with name '{name}' not found");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Role not found")
                    .await;
                return Err(ServiceError::NotFound(format!(
                    "Role with name '{name}' not found"
                )));
            }
            Err(e) => {
                error!("❌ Database error while finding role by name '{name}': {e:?}",);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Role retrieved by name successfully".to_string(),
            data: RoleResponse::from(role),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
