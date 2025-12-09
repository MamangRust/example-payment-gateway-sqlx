use crate::{
    abstract_trait::transfer::{
        repository::query::DynTransferQueryRepository, service::query::TransferQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transfer::FindAllTransfers,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TransferResponse,
            TransferResponseDeleteAt,
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

pub struct TransferQueryService {
    pub query: DynTransferQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferQueryService {
    pub fn new(query: DynTransferQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transfer-query-service")
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
impl TransferQueryServiceTrait for TransferQueryService {
    async fn find_all(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all transfers | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_transfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransferResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} transfers in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transfers, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} transfers", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all transfers: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch all transfers: {e:?}"),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Transfers retrieved successfully".to_string(),
            data: transfer_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} transfers (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError> {
        info!("🔍 Finding transfer by ID: {}", transfer_id);

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_transfer_by_id",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("transfer_id", transfer_id.to_string()),
            ],
        );

        let mut request = Request::new(transfer_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:find_by_id:{}", transfer_id);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<TransferResponse>>(&cache_key)
            .await
        {
            info!("✅ Found transfer with ID {transfer_id} in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Transfer retrieved from cache")
                .await;
            return Ok(cache);
        }

        let transfer = match self.query.find_by_id(transfer_id).await {
            Ok(transfer) => {
                info!("✅ Found transfer with ID: {transfer_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transfer retrieved successfully",
                )
                .await;
                transfer
            }
            Err(e) => {
                error!("❌ Database error fetching transfer ID {transfer_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error fetching transfer: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Transfer retrieved successfully".to_string(),
            data: TransferResponse::from(transfer),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all active transfers | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_active_transfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:find_by_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransferResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active transfers in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transfers, total_items) = match self.query.find_by_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active transfers", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active transfers: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch active transfers: {e:?}"),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponseDeleteAt> = transfers
            .into_iter()
            .map(TransferResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active transfers retrieved successfully".to_string(),
            data: transfer_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} active transfers (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all trashed transfers | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_trashed_transfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransferResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} trashed transfers in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transfers, total_items) = match self.query.find_by_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed transfers", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed transfers: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch trashed transfers: {e:?}"),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponseDeleteAt> = transfers
            .into_iter()
            .map(TransferResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed transfers retrieved successfully".to_string(),
            data: transfer_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} trashed transfers (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError> {
        if transfer_from.to_string().trim().is_empty() {
            return Err(ServiceError::Custom(
                "Transfer from account is required".to_string(),
            ));
        }

        info!("📤 Fetching transfers sent from: {transfer_from}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_transfers_from",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_transfer_from"),
                KeyValue::new("transfer_from", transfer_from.to_string()),
            ],
        );

        let mut request = Request::new(transfer_from);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:find_by_transfer_from:{}", transfer_from);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found transfers from {transfer_from} in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Transfers from account retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let transfers = match self.query.find_by_transfer_from(transfer_from).await {
            Ok(transfers) => {
                info!(
                    "✅ Found {} transfers sent from: {transfer_from}",
                    transfers.len(),
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transfers from account retrieved successfully",
                )
                .await;
                transfers
            }
            Err(e) => {
                error!("❌ Failed to fetch transfers from {transfer_from}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch transfers from account: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Transfers from account retrieved successfully".to_string(),
            data: transfer_responses,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError> {
        if transfer_to.to_string().trim().is_empty() {
            return Err(ServiceError::Custom(
                "Transfer to account is required".to_string(),
            ));
        }

        info!("📥 Fetching transfers sent to: {transfer_to}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_transfers_to",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_transfer_to"),
                KeyValue::new("transfer_to", transfer_to.to_string()),
            ],
        );

        let mut request = Request::new(transfer_to);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:find_by_transfer_to:{}", transfer_to);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found transfers to {transfer_to} in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Transfers to account retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let transfers = match self.query.find_by_transfer_to(transfer_to).await {
            Ok(transfers) => {
                info!(
                    "✅ Found {} transfers sent to: {transfer_to}",
                    transfers.len(),
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transfers to account retrieved successfully",
                )
                .await;
                transfers
            }
            Err(e) => {
                error!("❌ Failed to fetch transfers to {transfer_to}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch transfers to account: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Transfers to account retrieved successfully".to_string(),
            data: transfer_responses,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
