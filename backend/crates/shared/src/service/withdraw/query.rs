use crate::{
    abstract_trait::withdraw::{
        repository::query::DynWithdrawQueryRepository, service::query::WithdrawQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, WithdrawResponse,
            WithdrawResponseDeleteAt,
        },
    },
    errors::ServiceError,
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
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

pub struct WithdrawQueryService {
    pub query: DynWithdrawQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawQueryService {
    pub fn new(query: DynWithdrawQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("withdraw-query-service")
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
impl WithdrawQueryServiceTrait for WithdrawQueryService {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all withdrawals | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "withdrawal:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<WithdrawResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} withdrawals in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (withdrawals, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} withdrawals", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all withdrawals: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch all withdrawals: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponse> = withdrawals
            .into_iter()
            .map(WithdrawResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} withdrawals (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "💳 Fetching withdrawals for card number: {} | Page: {}, Size: {}, Search: {:?}",
            req.card_number, page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_withdrawals_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_all_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "withdrawal:find_all_by_card:card:{}:page:{page}:size:{page_size}:search:{}",
            req.card_number,
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<WithdrawResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} withdrawals for card {} in cache",
                cache.data.len(),
                req.card_number
            );
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (withdrawals, total_items) = match self.query.find_all_by_card_number(req).await {
            Ok(res) => {
                let log_msg = format!(
                    "✅ Found {} withdrawals for card {}",
                    res.0.len(),
                    req.card_number
                );
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch withdrawals for card {}: {e:?}",
                    req.card_number
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch withdrawals for card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponse> = withdrawals
            .into_iter()
            .map(WithdrawResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Withdrawals by card number retrieved successfully".to_string(),
            data: withdraw_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} withdrawals for card {} (total: {total_items})",
            response.data.len(),
            req.card_number
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        info!("🔍 Finding withdrawal by ID: {withdraw_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_withdrawal_by_id",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("withdraw_id", withdraw_id.to_string()),
            ],
        );

        let mut request_obj = Request::new(withdraw_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!("withdrawal:find_by_id:{}", withdraw_id);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<WithdrawResponse>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found withdrawal with ID {withdraw_id} in cache");
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let withdrawal = match self.query.find_by_id(withdraw_id).await {
            Ok(withdrawal) => {
                let log_msg = format!("✅ Found withdrawal with ID: {withdraw_id}");
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                withdrawal
            }
            Err(e) => {
                error!("❌ Database error fetching withdrawal ID {withdraw_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error fetching withdrawal: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Withdrawal retrieved successfully".to_string(),
            data: WithdrawResponse::from(withdrawal),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<WithdrawResponse>>, ServiceError> {
        info!("🔍 Finding withdrawals by card_number: {card_number}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_withdrawals_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_by_card"),
                KeyValue::new("card_number", card_number.to_string()),
            ],
        );

        let mut request_obj = Request::new(card_number);
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!("withdrawal:find_by_card:{}", card_number);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} withdrawals for card {card_number} in cache",
                cache.data.len()
            );
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let withdrawals = match self.query.find_by_card(card_number).await {
            Ok(withdrawals) => {
                let log_msg = format!(
                    "✅ Found {} withdrawals for card_number: {card_number}",
                    withdrawals.len()
                );
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                withdrawals
            }
            Err(e) => {
                error!(
                    "❌ Database error fetching withdrawals for card_number {card_number}: {e:?}"
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error fetching withdrawals for card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Withdrawals retrieved successfully".to_string(),
            data: withdrawals
                .into_iter()
                .map(WithdrawResponse::from)
                .collect(),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🟢 Fetching active withdrawals | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_active_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_by_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "withdrawal:find_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active withdrawals in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (withdrawals, total_items) = match self.query.find_by_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active withdrawals", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active withdrawals: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch active withdrawals: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponseDeleteAt> = withdrawals
            .into_iter()
            .map(WithdrawResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
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
            "✅ Found {} active withdrawals (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️ Fetching trashed withdrawals | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_trashed_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "find_by_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request_obj = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let cache_key = format!(
            "withdrawal:find_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed withdrawals in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (withdrawals, total_items) = match self.query.find_by_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed withdrawals", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed withdrawals: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch trashed withdrawals: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponseDeleteAt> = withdrawals
            .into_iter()
            .map(WithdrawResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
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
            "✅ Found {} trashed withdrawals (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }
}
