use crate::{
    abstract_trait::saldo::{
        repository::query::DynSaldoQueryRepository, service::query::SaldoQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::saldo::FindAllSaldos,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, SaldoResponse, SaldoResponseDeleteAt,
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

pub struct SaldoQueryService {
    pub query: DynSaldoQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl SaldoQueryService {
    pub fn new(query: DynSaldoQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("saldo-query-service")
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
impl SaldoQueryServiceTrait for SaldoQueryService {
    async fn find_all(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all saldos | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_all_saldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "saldo:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<SaldoResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} saldos in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (saldos, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} saldos", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all saldos: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch all saldos: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponse> =
            saldos.into_iter().map(SaldoResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Saldos retrieved successfully".to_string(),
            data: saldo_responses,
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
            "✅ Found {} saldos (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_active(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching active saldos | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_active_saldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "saldo:find_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active saldos in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (saldos, total_items) = match self.query.find_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active saldos", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active saldos: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch active saldos: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponseDeleteAt> = saldos
            .into_iter()
            .map(SaldoResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active saldos retrieved successfully".to_string(),
            data: saldo_responses,
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
            "✅ Found {} active saldos (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_trashed(
        &self,
        req: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️  Searching trashed saldos | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_trashed_saldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "saldo:find_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed saldos in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (saldos, total_items) = match self.query.find_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed saldos", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed saldos: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch trashed saldos: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let saldo_responses: Vec<SaldoResponseDeleteAt> = saldos
            .into_iter()
            .map(SaldoResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed saldos retrieved successfully".to_string(),
            data: saldo_responses,
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
            "🗑️  Found {} trashed saldos (total: {total_items})",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        let masked_card = mask_card_number(card_number);
        info!("💳 Finding saldo by card_number={masked_card}");

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_saldo_by_card",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_by_card"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut request = Request::new(card_number.to_string());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("saldo:find_by_card:card_number:{}", masked_card);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<SaldoResponse>>(&cache_key)
            .await
        {
            info!("✅ Found saldo in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Saldo retrieved from cache")
                .await;
            return Ok(cache);
        }

        let saldo = match self.query.find_by_card(card_number).await {
            Ok(saldo) => {
                info!(
                    "✅ Found saldo for card_number={masked_card}, id={}",
                    saldo.saldo_id
                );
                self.complete_tracing_success(&tracing_ctx, method, "Saldo retrieved successfully")
                    .await;
                saldo
            }
            Err(e) => {
                error!(
                    "❌ Database error while finding saldo by card_number={masked_card}: {:?}",
                    e
                );
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Saldo retrieved successfully".to_string(),
            data: SaldoResponse::from(saldo),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        info!("🔍 Finding saldo by ID: {id}");

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_saldo_by_id",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("saldo:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<SaldoResponse>>(&cache_key)
            .await
        {
            info!("✅ Found saldo in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Saldo retrieved from cache")
                .await;
            return Ok(cache);
        }

        let saldo = match self.query.find_by_id(id).await {
            Ok(saldo) => {
                info!("✅ Found saldo with ID: {id}");
                self.complete_tracing_success(&tracing_ctx, method, "Saldo retrieved successfully")
                    .await;
                saldo
            }
            Err(e) => {
                error!("❌ Database error while finding saldo by ID {id}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Saldo retrieved successfully".to_string(),
            data: SaldoResponse::from(saldo),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
