use crate::{
    abstract_trait::user::{
        repository::query::DynUserQueryRepository, service::query::UserQueryServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::user::FindAllUserRequest,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, UserResponse, UserResponseDeleteAt,
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

pub struct UserQueryService {
    pub query: DynUserQueryRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl UserQueryService {
    pub fn new(query: DynUserQueryRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            query,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl UserQueryServiceTrait for UserQueryService {
    async fn find_all(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all users | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_all_users",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "user:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} users in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (users, total_items) = match self.query.find_all(req.clone()).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} users", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all users: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch all users: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Users retrieved successfully".to_string(),
            data: user_responses,
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
            "✅ Found {} users (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!("🔍 Finding user by ID: {user_id}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_user_by_id",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("user:find_by_id:{}", user_id);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<UserResponse>>(&cache_key)
            .await
        {
            info!("✅ Found user with ID {user_id} in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "User retrieved from cache")
                .await;
            return Ok(cache);
        }

        let user = match self.query.find_by_id(user_id).await {
            Ok(user) => {
                info!("✅ Found user with ID: {user_id}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "User retrieved successfully")
                    .await;
                user
            }
            Err(e) => {
                error!("❌ Database error fetching user ID {user_id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Database error fetching user: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "User retrieved successfully".to_string(),
            data: UserResponse::from(user),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_active(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🟢 Fetching active users | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_active_users",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_by_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "user:find_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active users in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (users, total_items) = match self.query.find_by_active(req.clone()).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active users", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active users: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch active users: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponseDeleteAt> =
            users.into_iter().map(UserResponseDeleteAt::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active users retrieved successfully".to_string(),
            data: user_responses,
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
            "✅ Found {} active users (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️ Fetching trashed users | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_trashed_users",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "find_by_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "user:find_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<UserResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed users in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (users, total_items) = match self.query.find_by_trashed(req.clone()).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed users", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed users: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch trashed users: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponseDeleteAt> =
            users.into_iter().map(UserResponseDeleteAt::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed users retrieved successfully".to_string(),
            data: user_responses,
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
            "✅ Found {} trashed users (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }
}
