use crate::{
    abstract_trait::merchant::{
        repository::query::DynMerchantQueryRepository, service::query::MerchantQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::merchant::FindAllMerchants,
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
            Pagination,
        },
    },
    errors::{RepositoryError, ServiceError},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_api_key,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::Request;
use tracing::{error, info};

pub struct MerchantQueryService {
    pub query: DynMerchantQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantQueryService {
    pub fn new(query: DynMerchantQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("merchant-query-service")
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
            info!("✅ Operation completed successfully: {message}");
        } else {
            error!("❌ Operation failed: {message}");
        }

        self.metrics.record(method, status, elapsed);

        tracing_ctx.cx.span().end();
    }
}

#[async_trait]
impl MerchantQueryServiceTrait for MerchantQueryService {
    async fn find_all(
        &self,
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_merchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:find_all:page:{page}:size:{page_size}:search:{}",
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchants in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Merchants retrieved from cache")
                .await;
            return Ok(cache);
        }

        let (merchants, total_items) = match self.query.find_all(req).await {
            Ok((merchants, total_items)) => {
                info!("✅ Found {} merchants", merchants.len());
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchants retrieved successfully",
                )
                .await;
                (merchants, total_items)
            }
            Err(e) => {
                error!("❌ Failed to fetch all merchants: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch all merchants: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponse> =
            merchants.into_iter().map(MerchantResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Merchants retrieved successfully".to_string(),
            data: merchant_responses,
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

        Ok(response)
    }

    async fn find_active(
        &self,
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "✅ Fetching active merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_active_merchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:find_active:page:{page}:size:{page_size}:search:{}",
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found active merchants in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Active merchants retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let (merchants, total_items) = match self.query.find_active(req).await {
            Ok((merchants, total_items)) => {
                info!("✅ Retrieved {} active merchants", merchants.len());
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Active merchants retrieved successfully",
                )
                .await;
                (merchants, total_items)
            }
            Err(e) => {
                error!("❌ Failed to fetch active merchants: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch active merchants: {:?}", e),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponseDeleteAt> = merchants
            .into_iter()
            .map(MerchantResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active merchants retrieved successfully".to_string(),
            data: merchant_responses,
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

        Ok(response)
    }

    async fn find_trashed(
        &self,
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️  Fetching trashed merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_trashed_merchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:find_trashed:page:{page}:size:{page_size}:search:{}",
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found trashed merchants in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Trashed merchants retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let (merchants, total_items) = match self.query.find_trashed(req).await {
            Ok((merchants, total_items)) => {
                info!("🗑️  Found {} trashed merchants", merchants.len());
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Trashed merchants retrieved successfully",
                )
                .await;
                (merchants, total_items)
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed merchants: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch trashed merchants: {:?}", e),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponseDeleteAt> = merchants
            .into_iter()
            .map(MerchantResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed merchants retrieved successfully".to_string(),
            data: merchant_responses,
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

        Ok(response)
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        info!("🔍 Finding merchant by ID: {}", id);

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_merchant_by_id",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<MerchantResponse>>(&cache_key)
            .await
        {
            info!("✅ Found merchant in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Merchant retrieved from cache")
                .await;
            return Ok(cache);
        }

        let merchant = match self.query.find_by_id(id).await {
            Ok(merchant) => {
                info!("✅ Merchant retrieved successfully (ID: {id})");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant retrieved successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                let error_msg = match e {
                    RepositoryError::NotFound => {
                        info!("ℹ️  Merchant with ID {id} not found");
                        "Merchant not found"
                    }
                    _ => {
                        error!("❌ Database error while finding merchant by ID {id}: {e:?}",);
                        "Database error"
                    }
                };

                self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                    .await;

                return match e {
                    RepositoryError::NotFound => {
                        Err(ServiceError::NotFound("Merchant not found".to_string()))
                    }
                    _ => Err(ServiceError::InternalServerError(e.to_string())),
                };
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Merchant retrieved successfully".to_string(),
            data: MerchantResponse::from(merchant),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        let masked_key = mask_api_key(api_key);

        info!("🔑 Finding merchant by API key: {masked_key}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_merchant_by_apikey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_apikey"),
                KeyValue::new("api_key", masked_key.clone()),
            ],
        );

        let mut request = Request::new(api_key.to_string());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:find_by_apikey:key:{masked_key}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<MerchantResponse>>(&cache_key)
            .await
        {
            info!("✅ Found merchant in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Merchant retrieved from cache")
                .await;
            return Ok(cache);
        }

        let merchant = match self.query.find_by_apikey(api_key).await {
            Ok(merchant) => {
                info!("✅ Merchant found for API key: {masked_key}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant retrieved successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                let error_msg = match e {
                    RepositoryError::NotFound => {
                        info!("ℹ️  No merchant found for API key: {masked_key}");
                        "Invalid API key"
                    }
                    _ => {
                        error!("❌ Error fetching merchant by API key {masked_key}: {e:?}",);
                        "Database error"
                    }
                };

                self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                    .await;

                return match e {
                    RepositoryError::NotFound => {
                        Err(ServiceError::NotFound("Invalid API key".to_string()))
                    }
                    _ => Err(ServiceError::InternalServerError(e.to_string())),
                };
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Merchant retrieved by API key".to_string(),
            data: MerchantResponse::from(merchant),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError> {
        info!("👥 Finding merchants for user ID: {user_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_merchants_by_user_id",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_user_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:find_by_user_id:user_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchants for user in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Merchants for user retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let merchants = match self.query.find_merchant_user_id(user_id).await {
            Ok(merchants) => {
                info!(
                    "✅ Found {} merchants for user ID {user_id}",
                    merchants.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchants for user retrieved successfully",
                )
                .await;
                merchants
            }
            Err(e) => {
                error!("❌ Failed to fetch merchants for user ID {user_id}: {e:?}",);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch merchants for user: {:?}", e),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let merchant_responses: Vec<MerchantResponse> =
            merchants.into_iter().map(MerchantResponse::from).collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Merchants by user ID retrieved successfully".to_string(),
            data: merchant_responses,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
