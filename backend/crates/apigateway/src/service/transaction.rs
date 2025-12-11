use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::transaction::{
    CreateTransactionRequest, FindAllTransactionCardNumberRequest, FindAllTransactionRequest,
    FindByIdTransactionRequest, FindByYearCardNumberTransactionRequest,
    FindMonthlyTransactionStatus, FindMonthlyTransactionStatusCardNumber,
    FindTransactionByMerchantIdRequest, FindYearTransactionStatus,
    FindYearTransactionStatusCardNumber, UpdateTransactionRequest,
    transaction_service_client::TransactionServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::cache::CacheStore;
use shared::{
    abstract_trait::transaction::http::{
        TransactionCommandGrpcClientTrait, TransactionGrpcClientServiceTrait,
        TransactionQueryGrpcClientTrait, TransactionStatsAmountByCardNumberGrpcClientTrait,
        TransactionStatsAmountGrpcClientTrait, TransactionStatsMethodByCardNumberGrpcClientTrait,
        TransactionStatsMethodGrpcClientTrait, TransactionStatsStatusByCardNumberGrpcClientTrait,
        TransactionStatsStatusGrpcClientTrait,
    },
    domain::{
        requests::transaction::{
            CreateTransactionRequest as DomainCreateTransactionRequest,
            FindAllTransactionCardNumber, FindAllTransactions as DomainFindAllTransactions,
            MonthStatusTransaction as DomainMonthStatusTransaction,
            MonthStatusTransactionCardNumber as DomainMonthStatusTransactionCardNumber,
            MonthYearPaymentMethod as DomainMonthYearPaymentMethod,
            UpdateTransactionRequest as DomainUpdateTransactionRequest,
            YearStatusTransactionCardNumber as DomainYearStatusTransactionCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransactionMonthAmountResponse,
            TransactionMonthMethodResponse, TransactionResponse, TransactionResponseDeleteAt,
            TransactionResponseMonthStatusFailed, TransactionResponseMonthStatusSuccess,
            TransactionResponseYearStatusFailed, TransactionResponseYearStatusSuccess,
            TransactionYearMethodResponse, TransactionYearlyAmountResponse,
        },
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_api_key,
        mask_card_number, month_name, naive_datetime_to_timestamp,
    },
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct TransactionGrpcClientService {
    client: TransactionServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl TransactionGrpcClientService {
    pub fn new(
        client: TransactionServiceClient<Channel>,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transaction-client-service")
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
impl TransactionGrpcClientServiceTrait for TransactionGrpcClientService {}

#[async_trait]
impl TransactionQueryGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching all transactions - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransactionRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:find_all:page:{page}:size:{page_size}:search:{}",
            req.search
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} transactions in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_all_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transactions",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
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

                info!("fetched {} transactions", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch transactions")
                    .await;
                error!("fetch all transactions failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);

        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching transactions for card: {} - page: {page}, page_size: {page_size}, search: {:?}",
            masked_card, req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllTransactionByCardNumber",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_all_by_card_number"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransactionCardNumberRequest {
            card_number: req.card_number.clone(),
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:find_by_card_number:card:{}:page:{page}:size:{page_size}:search:{}",
            masked_card, req.card_number,
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} transactions in cache for card: {}",
                cache.data.len(),
                masked_card
            );
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_all_transaction_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transactions by card number",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
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

                info!(
                    "fetched {} transactions for card {}",
                    api_response.data.len(),
                    masked_card
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch transactions by card number",
                )
                .await;
                error!(
                    "fetch transactions for card {} failed: {status:?}",
                    masked_card
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching active transactions - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransactionRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:find_by_active:page:{page}:size:{page_size}:search:{}",
            req.search,
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active transactions in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_by_active_transaction(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active transactions",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseDeleteAt> =
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

                info!("fetched {} active transactions", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch active transactions",
                )
                .await;
                error!("fetch active transactions failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching trashed transactions - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransactionRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            req.search
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} trashed transactions in cache",
                cache.data.len()
            );
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_by_trashed_transaction(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed transactions",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseDeleteAt> =
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

                info!("fetched {} trashed transactions", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch trashed transactions",
                )
                .await;
                error!("fetch trashed transactions failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, HttpError> {
        info!("fetching transaction by id: {transaction_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTransactionById",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:find_by_id:id:{transaction_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<TransactionResponse>>(&cache_key)
            .await
        {
            info!("✅ Found transaction in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Transaction retrieved from cache")
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_id_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transaction by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transaction {transaction_id} - data missing in gRPC response");
                    HttpError::Internal("Transaction data is missing in gRPC response".into())
                })?;

                let api_response = ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found transaction {transaction_id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch transaction by id",
                )
                .await;
                error!("find transaction {transaction_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, HttpError> {
        info!("fetching transactions by merchant_id: {merchant_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTransactionByMerchantId",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_by_merchant_id"),
                KeyValue::new("merchant_id", merchant_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindTransactionByMerchantIdRequest { merchant_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:find_by_merchant_id:merchant_id:{merchant_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found transactions for merchant in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Transactions for merchant retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_transaction_by_merchant_id(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transactions by merchant id",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} transactions for merchant {merchant_id}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch transactions by merchant id",
                )
                .await;
                error!("fetch transactions for merchant {merchant_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionCommandGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, api_key, req), level = "info")]
    async fn create(
        &self,
        api_key: &str,
        req: &DomainCreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, HttpError> {
        let masked_api = mask_api_key(api_key);
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating transaction via api_key: {masked_api} for card: {masked_card}, amount: {}, merchant_id: {:?}, method: {}",
            req.amount, req.merchant_id, req.payment_method
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "create"),
                KeyValue::new("api_key", masked_api.clone()),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("merchant_id", req.merchant_id.unwrap_or(0).to_string()),
            ],
        );

        let date = naive_datetime_to_timestamp(req.transaction_time);

        let mut grpc_req = Request::new(CreateTransactionRequest {
            api_key: api_key.to_string(),
            card_number: req.card_number.clone(),
            amount: req.amount,
            payment_method: req.payment_method.clone(),
            merchant_id: req.merchant_id.unwrap_or(0),
            transaction_time: Some(date),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully created transaction",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transaction creation failed - data missing in gRPC response for card: {masked_card}");
                    HttpError::Internal("Transaction data is missing in gRPC response".into())
                })?;

                let transaction_response: TransactionResponse = data.into();

                let api_response = ApiResponse {
                    data: transaction_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    format!("transaction:find_by_card:{}", req.card_number),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let cache_key = vec![
                    format!("transaction:find_by_card:{}", req.card_number),
                    format!("transaction:find_by_id:{}", api_response.data.clone().id),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!("transaction created successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create transaction")
                    .await;
                error!(
                    "create transaction for card {masked_card} via api_key {masked_api} failed: {status:?}"
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, api_key, req), level = "info")]
    async fn update(
        &self,
        api_key: &str,
        req: &DomainUpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, HttpError> {
        let masked_api = mask_api_key(api_key);
        let masked_card = mask_card_number(&req.card_number);

        let transaction_id = req
            .transaction_id
            .ok_or_else(|| HttpError::Internal("transaction_id is required".to_string()))?;

        info!(
            "updating transaction id: {transaction_id} via api_key: {masked_api} for card: {masked_card}, new amount: {}, method: {}",
            req.amount, req.payment_method
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "update"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
                KeyValue::new("api_key", masked_api.clone()),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let date = naive_datetime_to_timestamp(req.transaction_time);

        let mut grpc_req = Request::new(UpdateTransactionRequest {
            transaction_id,
            api_key: api_key.to_string(),
            card_number: req.card_number.clone(),
            amount: req.amount as i32,
            payment_method: req.payment_method.clone(),
            merchant_id: req.merchant_id.unwrap_or(0),
            transaction_time: Some(date),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully updated transaction",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update transaction {transaction_id} - data missing in gRPC response");
                    HttpError::Internal("Transaction data is missing in gRPC response".into())
                })?;

                let transaction_response: TransactionResponse = data.into();

                let api_response = ApiResponse {
                    data: transaction_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    format!("transaction:find_by_card:{}", req.card_number),
                    format!("transaction:find_by_id:{}", api_response.data.clone().id),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let cache_key = vec![
                    format!("transaction:find_by_card:{}", req.card_number),
                    format!("transaction:find_by_id:{}", api_response.data.clone().id),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!(
                    "transaction {transaction_id} updated successfully for card: {}",
                    masked_card
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update transaction")
                    .await;
                error!(
                    "update transaction {transaction_id} via api_key {masked_api} failed: {status:?}",
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, HttpError> {
        info!("trashing transaction id: {transaction_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully trashed transaction",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash transaction {transaction_id} - data missing in gRPC response");
                    HttpError::Internal("Transaction data is missing in gRPC response".into())
                })?;

                let api_response = ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "transaction;find_by_id:*".to_string(),
                    "transaction:find_by_card:*".to_string(),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("transaction {transaction_id} trashed successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash transaction")
                    .await;
                error!("trash transaction {transaction_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, HttpError> {
        info!("restoring transaction id: {transaction_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored transaction",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore transaction {transaction_id} - data missing in gRPC response");
                    HttpError::Internal("Transaction data is missing in gRPC response".into())
                })?;

                let cache_keys = vec![
                    "transaction:find_by_id:*".to_string(),
                    "transaction:find_by_card:*".to_string(),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let api_response = ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                };

                info!("transaction {transaction_id} restored successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore transaction")
                    .await;
                error!("restore transaction {transaction_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, transaction_id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting transaction id: {transaction_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteTransactionPermanent",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_transaction_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted transaction permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "transaction:find_by_id:*".to_string(),
                    "transaction:find_by_card:*".to_string(),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("transaction {transaction_id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete transaction permanently",
                )
                .await;
                error!("delete transaction {transaction_id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed transactions");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllTransactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_transaction(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all trashed transactions",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "transaction:find_by_id:*".to_string(),
                    "transaction:find_by_card:*".to_string(),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("all trashed transactions restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all trashed transactions",
                )
                .await;
                error!("restore all transactions failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all transactions");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllTransactionsPermanent",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_transaction_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted all transactions permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "transaction:find_by_id:*".to_string(),
                    "transaction:find_by_card:*".to_string(),
                    "transaction:find_all:*".to_string(),
                    "transaction:find_by_active:*".to_string(),
                    "transaction:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("all transactions permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all transactions permanently",
                )
                .await;
                error!("delete all transactions permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsAmountGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, HttpError> {
        info!("fetching monthly transaction AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transaction amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transaction amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().find_monthly_amounts(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction amounts",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} monthly transaction amount records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction amounts",
                )
                .await;
                error!("fetch monthly transaction AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, HttpError> {
        info!("fetching yearly transaction AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transaction amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transaction amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().find_yearly_amounts(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction amounts",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} yearly transaction amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction amounts",
                )
                .await;
                error!("fetch yearly transaction AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsMethodGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, HttpError> {
        info!("fetching monthly transaction METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyMethodTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_monthly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:monthly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transaction methods in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transaction methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_payment_methods(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction methods",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} monthly transaction method records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction methods",
                )
                .await;
                error!("fetch monthly transaction METHOD for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, HttpError> {
        info!("fetching yearly transaction METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyMethodTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:yearly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transaction methods in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transaction methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_payment_methods(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction methods",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionYearMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} yearly transaction method records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction methods",
                )
                .await;
                error!("fetch yearly transaction METHOD for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsStatusGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction SUCCESS status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccessTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_month_status_success"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransactionStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transactions in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transaction_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction success status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly SUCCESS transaction records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction success status",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS transaction status for {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, HttpError> {
        info!("fetching yearly transaction SUCCESS status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful transactions in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transaction_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction success status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} yearly SUCCESS transaction records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction success status",
                )
                .await;
                error!(
                    "fetch yearly SUCCESS transaction status for year {year} failed: {status:?}"
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction FAILED status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailedTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_month_status_failed"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransactionStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transactions in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transaction_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction failed status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED transaction records for {month_str} {}",
                    data.len(),
                    req.year
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction failed status",
                )
                .await;
                error!(
                    "fetch monthly FAILED transaction status for {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, HttpError> {
        info!("fetching yearly transaction FAILED status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transaction:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed transactions in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transaction_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction failed status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} yearly FAILED transaction records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction failed status",
                )
                .await;
                error!("fetch yearly FAILED transaction status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsAmountByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transaction AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_monthly_amounts_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transaction amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction amounts by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} monthly transaction amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction amounts by card",
                )
                .await;
                error!(
                    "fetch monthly transaction AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_amounts_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:yearly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transaction amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction amounts by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} yearly transaction amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction amounts by card",
                )
                .await;
                error!(
                    "fetch yearly transaction AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsMethodByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transaction METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyMethodsByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_monthly_method_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction methods in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transaction methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_payment_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction methods by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly transaction method records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction methods by card",
                )
                .await;
                error!(
                    "fetch monthly transaction METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyMethodsByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_method_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:yearly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction methods in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transaction methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_payment_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction methods by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionYearMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} yearly transaction method records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction methods by card",
                )
                .await;
                error!(
                    "fetch yearly transaction METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransactionStatsStatusByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccessByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_month_status_success_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_status_success:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transactions in cache for card: {} ({}-{})",
                masked_card, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transaction_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction success status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly SUCCESS transaction records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction success status by card",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS transaction status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_status_success_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:yearly_status_success:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful transactions in cache for card: {} ({})",
                masked_card, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transaction_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction success status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} yearly SUCCESS transaction records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction success status by card",
                )
                .await;
                error!(
                    "fetch yearly SUCCESS transaction status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailedByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_month_status_failed_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:monthly_status_failed:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transactions in cache for card: {} ({}-{})",
                masked_card, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transaction_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction failed status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly FAILED transaction records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction failed status by card",
                )
                .await;
                error!(
                    "fetch monthly FAILED transaction status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedByCardTransaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "get_yearly_status_failed_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transaction:yearly_status_failed:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed transactions in cache for card: {} ({})",
                masked_card, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transaction_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction failed status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} yearly FAILED transaction records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction failed status by card",
                )
                .await;
                error!(
                    "fetch yearly FAILED transaction status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
