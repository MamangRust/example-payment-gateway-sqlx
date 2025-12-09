use crate::{
    abstract_trait::topup::{
        repository::query::DynTopupQueryRepository, service::query::TopupQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TopupResponse, TopupResponseDeleteAt,
        },
    },
    errors::ServiceError,
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_card_number,
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

pub struct TopupQueryService {
    pub query: DynTopupQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupQueryService {
    pub fn new(query: DynTopupQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("topup-query-service")
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
impl TopupQueryServiceTrait for TopupQueryService {
    async fn find_all(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all topups | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_topups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TopupResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} topups in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (topups, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} topups", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all topups: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch all topups: {e:?}"),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponse> =
            topups.into_iter().map(TopupResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Topups retrieved successfully".to_string(),
            data: topup_responses,
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
            "✅ Found {} topups (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError> {
        if req.card_number.trim().is_empty() {
            return Err(ServiceError::Custom("Card number is required".to_string()));
        }

        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let masked_card = mask_card_number(&req.card_number);
        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "💳 Searching topups by card number: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            masked_card, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_topups_by_card_number",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_by_card_number"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:find_by_card_number:card:{}:page:{page}:size:{page_size}:search:{}",
            masked_card,
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TopupResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} topups in cache for card {}",
                cache.data.len(),
                masked_card
            );
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (topups, total_items) = match self.query.find_all_by_card_number(req).await {
            Ok(res) => {
                let log_msg = format!(
                    "✅ Found {} topup records for card {}",
                    res.0.len(),
                    masked_card
                );
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch topups for card number {}: {e:?}",
                    masked_card
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch topups for card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponse> =
            topups.into_iter().map(TopupResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Topups by card number retrieved successfully".to_string(),
            data: topup_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} topups for card {} (total: {total_items})",
            response.data.len(),
            masked_card
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🔍 Finding topup by ID: {topup_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_topup_by_id",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut request = Request::new(topup_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:find_by_id:id:{topup_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<TopupResponse>>(&cache_key)
            .await
        {
            info!("✅ Found topup in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Topup retrieved from cache")
                .await;
            return Ok(cache);
        }

        let topup = match self.query.find_by_id(topup_id).await {
            Ok(topup) => {
                info!("✅ Found topup with ID: {topup_id}");
                self.complete_tracing_success(&tracing_ctx, method, "Topup retrieved successfully")
                    .await;
                topup
            }

            Err(e) => {
                error!("❌ Database error while finding topup ID {topup_id}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Topup retrieved successfully".to_string(),
            data: TopupResponse::from(topup),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, ServiceError> {
        let masked_card = mask_card_number(card_number);
        info!("🔍 Finding topups by card_number: {masked_card}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_topups_by_card",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_by_card"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut request = Request::new(card_number.to_string());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:find_by_card:card_number:{}", masked_card);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found topups in cache for card: {masked_card}");
            self.complete_tracing_success(&tracing_ctx, method, "Topups retrieved from cache")
                .await;
            return Ok(cache);
        }

        let topups = match self.query.find_by_card(card_number).await {
            Ok(topups) => {
                info!("✅ Found {} topups for card {masked_card}", topups.len());
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Topups retrieved successfully",
                )
                .await;
                topups
            }
            Err(e) => {
                error!("❌ Database error while finding topups for card {masked_card}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Topups retrieved successfully".to_string(),
            data: topups.into_iter().map(TopupResponse::from).collect(),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_active(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🟢 Searching active topups | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_active_topups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:find_by_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TopupResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active topups in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (topups, total_items) = match self.query.find_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active topups", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active topups: {:?}", e);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch active topups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponseDeleteAt> = topups
            .into_iter()
            .map(TopupResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active topups retrieved successfully".to_string(),
            data: topup_responses,
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
            "✅ Found {} active topups (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_trashed(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️ Searching trashed topups | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_trashed_topups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TopupResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed topups in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (topups, total_items) = match self.query.find_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed topups", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed topups: {:?}", e);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch trashed topups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponseDeleteAt> = topups
            .into_iter()
            .map(TopupResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed topups retrieved successfully".to_string(),
            data: topup_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "🗑️ Found {} trashed topups (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
