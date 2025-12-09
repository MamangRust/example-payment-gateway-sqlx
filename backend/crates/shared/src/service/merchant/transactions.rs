use crate::{
    abstract_trait::merchant::{
        repository::transactions::DynMerchantTransactionRepository,
        service::transactions::MerchantTransactionServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::merchant::{
            FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById,
        },
        responses::{ApiResponsePagination, MerchantTransactionResponse, Pagination},
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

pub struct MerchantTransactionService {
    pub transaction: DynMerchantTransactionRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantTransactionService {
    pub fn new(
        transaction: DynMerchantTransactionRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            transaction,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("merchant-transaction-service")
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
impl MerchantTransactionServiceTrait for MerchantTransactionService {
    async fn find_all(
        &self,
        req: &FindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all merchant transactions | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_merchant_transactions",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant_transaction:find_all:page:{page}:size:{page_size}:search:{}",
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Merchant transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self.transaction.find_all_transactiions(req).await {
            Ok((transactions, total_items)) => {
                info!("✅ Found {} merchant transactions", transactions.len());
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant transactions retrieved successfully",
                )
                .await;
                (transactions, total_items)
            }
            Err(e) => {
                error!("❌ Failed to fetch all merchant transactions: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch all merchant transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Merchant transactions retrieved successfully".to_string(),
            data: response_data,
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

    async fn find_all_by_api_key(
        &self,
        req: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };
        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        let masked_key = mask_api_key(&req.api_key);

        info!(
            "🔑 Fetching transactions by API key | Key: {masked_key}, Page: {page}, Size: {page_size}, Search: {:?}",
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_merchant_transactions_by_api_key",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_by_api_key"),
                KeyValue::new("api_key", masked_key.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant_transaction:find_by_api_key:key:{masked_key}:page:{page}:size:{page_size}:search:{}",
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions by API key in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Merchant transactions by API key retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self
            .transaction
            .find_all_transactiions_by_api_key(req)
            .await
        {
            Ok((transactions, total_items)) => {
                info!(
                    "✅ Retrieved {} transactions for API key: {masked_key}",
                    transactions.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant transactions by API key retrieved successfully",
                )
                .await;
                (transactions, total_items)
            }
            Err(e) => {
                error!("❌ Failed to fetch transactions for API key {masked_key}: {e:?}",);
                let error_message = match e {
                    RepositoryError::NotFound => {
                        "No transactions found for this API key".to_string()
                    }
                    _ => e.to_string(),
                };

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to fetch transactions for API key: {}",
                        error_message
                    ),
                )
                .await;

                return match e {
                    RepositoryError::NotFound => Err(ServiceError::NotFound(
                        "No transactions found for this API key".to_string(),
                    )),
                    _ => Err(ServiceError::InternalServerError(e.to_string())),
                };
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by API key retrieved successfully".to_string(),
            data: response_data,
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

    async fn find_all_by_id(
        &self,
        req: &FindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🆔 Fetching transactions by merchant ID: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            req.merchant_id,
            search_str.clone()
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_all_merchant_transactions_by_id",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant_transaction:find_by_id:merchant_id:{}:page:{page}:size:{page_size}:search:{}",
            req.merchant_id,
            search_str.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions by ID in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Merchant transactions by ID retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let (transactions, total_items) =
            match self.transaction.find_all_transactiions_by_id(req).await {
                Ok((transactions, total_items)) => {
                    info!(
                        "✅ Found {} transactions for merchant ID {}",
                        transactions.len(),
                        req.merchant_id
                    );
                    self.complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Merchant transactions by ID retrieved successfully",
                    )
                    .await;
                    (transactions, total_items)
                }
                Err(e) => {
                    error!(
                        "❌ Failed to fetch transactions for merchant ID {}: {e:?}",
                        req.merchant_id
                    );
                    let error_message = match e {
                        RepositoryError::NotFound => {
                            "Merchant not found or has no transactions".to_string()
                        }
                        _ => e.to_string(),
                    };

                    self.complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to fetch transactions for merchant ID: {}",
                            error_message
                        ),
                    )
                    .await;

                    return match e {
                        RepositoryError::NotFound => Err(ServiceError::NotFound(
                            "Merchant not found or has no transactions".to_string(),
                        )),
                        _ => Err(ServiceError::InternalServerError(e.to_string())),
                    };
                }
            };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by merchant ID retrieved successfully".to_string(),
            data: response_data,
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
}
