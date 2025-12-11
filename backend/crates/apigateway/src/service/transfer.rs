use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::transfer::{
    CreateTransferRequest, FindAllTransferRequest, FindByCardNumberTransferRequest,
    FindByIdTransferRequest, FindMonthlyTransferStatus, FindMonthlyTransferStatusCardNumber,
    FindTransferByTransferFromRequest, FindTransferByTransferToRequest, FindYearTransferStatus,
    FindYearTransferStatusCardNumber, UpdateTransferRequest,
    transfer_service_client::TransferServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::transfer::http::{
        TransferCommandGrpcClientTrait, TransferGrpcClientServiceTrait,
        TransferQueryGrpcClientTrait, TransferStatsAmountByCardNumberGrpcClientTrait,
        TransferStatsAmountGrpcClientTrait, TransferStatsStatusByCardNumberGrpcClientTrait,
        TransferStatsStatusGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transfer::{
            CreateTransferRequest as DomainCreateTransferRequest,
            FindAllTransfers as DomainFindAllTransfers,
            MonthStatusTransfer as DomainMonthStatusTransfer,
            MonthStatusTransferCardNumber as DomainMonthStatusTransferCardNumber,
            MonthYearCardNumber as DomainMonthYearCardNumber,
            UpdateTransferRequest as DomainUpdateTransferRequest,
            YearStatusTransferCardNumber as DomainYearStatusTransferCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransferMonthAmountResponse, TransferResponse,
            TransferResponseDeleteAt, TransferResponseMonthStatusFailed,
            TransferResponseMonthStatusSuccess, TransferResponseYearStatusFailed,
            TransferResponseYearStatusSuccess, TransferYearAmountResponse,
        },
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_card_number,
        month_name,
    },
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct TransferGrpcClientService {
    client: TransferServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl TransferGrpcClientService {
    pub fn new(
        client: TransferServiceClient<Channel>,
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
        global::tracer("transfer-client-service")
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
impl TransferGrpcClientServiceTrait for TransferGrpcClientService {}

#[async_trait]
impl TransferQueryGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching all transfers - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransferRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:find_all:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_all_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transfers",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

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

                info!("fetched {} transfers", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch transfers")
                    .await;
                error!("fetch all transfers failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, HttpError> {
        info!("fetching transfer by id: {transfer_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTransferById",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("transfer_id", transfer_id.to_string()),
            ],
        );
        let cache_key = format!("transfer:find_by_id:{transfer_id}");

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

        let mut grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().find_by_id_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transfer by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transfer {transfer_id} - data missing in gRPC response");
                    HttpError::Internal("Transfer data is missing in gRPC response".into())
                })?;

                let transfer_response: TransferResponse = data.into();

                let api_response = ApiResponse {
                    data: transfer_response,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found transfer {transfer_id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch transfer by id")
                    .await;
                error!("find transfer {transfer_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching active transfers - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransferRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:find_by_active:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_by_active_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active transfers",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransferResponseDeleteAt> =
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

                info!("fetched {} active transfers", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch active transfers",
                )
                .await;
                error!("fetch active transfers failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching trashed transfers - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTransferRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_by_trashed_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed transfers",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransferResponseDeleteAt> =
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

                info!("fetched {} trashed transfers", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch trashed transfers",
                )
                .await;
                error!("fetch trashed transfers failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, transfer_from), level = "info")]
    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, HttpError> {
        let masked_from = mask_card_number(transfer_from);
        info!("fetching transfers FROM card: {masked_from}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTransferByTransferFrom",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_transfer_from"),
                KeyValue::new("transfer_from", masked_from.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindTransferByTransferFromRequest {
            transfer_from: transfer_from.to_string(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self
            .client
            .clone()
            .find_transfer_by_transfer_from(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transfers by transfer from",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} transfers from card {masked_from}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch transfers by transfer from",
                )
                .await;
                error!("fetch transfers FROM card {masked_from} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, transfer_to), level = "info")]
    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, HttpError> {
        let masked_to = mask_card_number(transfer_to);
        info!("fetching transfers TO card: {masked_to}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTransferByTransferTo",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "find_by_transfer_to"),
                KeyValue::new("transfer_to", masked_to.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindTransferByTransferToRequest {
            transfer_to: transfer_to.to_string(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self
            .client
            .clone()
            .find_transfer_by_transfer_to(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched transfers by transfer to",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} transfers to card {masked_to}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch transfers by transfer to",
                )
                .await;
                error!("fetch transfers TO card {masked_to} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransferCommandGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, HttpError> {
        let masked_from = mask_card_number(&req.transfer_from);
        let masked_to = mask_card_number(&req.transfer_to);
        info!(
            "creating transfer FROM {masked_from} TO {masked_to}, amount: {}",
            req.transfer_amount
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "create"),
                KeyValue::new("transfer.from", masked_from.clone()),
                KeyValue::new("transfer.to", masked_to.clone()),
                KeyValue::new("transfer.amount", req.transfer_amount.to_string()),
            ],
        );

        let mut grpc_req = Request::new(CreateTransferRequest {
            transfer_from: req.transfer_from.clone(),
            transfer_to: req.transfer_to.clone(),
            transfer_amount: req.transfer_amount as i32,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully created Transfer",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                error!("transfer creation failed - data missing in gRPC response FROM {masked_from} TO {masked_to}");
                HttpError::Internal("Transfer data is missing in gRPC response".into())
            })?;

                let transfer_response: TransferResponse = data.into();

                let api_response = ApiResponse {
                    data: transfer_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {key}");
                }

                let cache_key = vec![
                    format!("transfer:find_by_id:{}", api_response.data.clone().id),
                    format!(
                        "transfer:find_transfer_from:{}",
                        api_response.data.clone().transfer_from
                    ),
                    format!(
                        "transfer:find_transfer_to:{}",
                        api_response.data.clone().transfer_to
                    ),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!("transfer created successfully FROM {masked_from} TO {masked_to}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create Transfer")
                    .await;
                error!("create transfer FROM {masked_from} TO {masked_to} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, HttpError> {
        let masked_from = mask_card_number(&req.transfer_from);
        let masked_to = mask_card_number(&req.transfer_to);

        let transfer_id = req
            .transfer_id
            .ok_or_else(|| HttpError::Internal("transfer_id is required".to_string()))?;

        info!(
            "updating transfer id: {transfer_id} FROM {masked_from} TO {masked_to}, new amount: {}",
            req.transfer_amount
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "update"),
                KeyValue::new("transfer.id", transfer_id.to_string()),
                KeyValue::new("transfer.from", masked_from.clone()),
                KeyValue::new("transfer.to", masked_to.clone()),
                KeyValue::new("transfer.amount", req.transfer_amount.to_string()),
            ],
        );

        let mut grpc_req = Request::new(UpdateTransferRequest {
            transfer_id,
            transfer_from: req.transfer_from.clone(),
            transfer_to: req.transfer_to.clone(),
            transfer_amount: req.transfer_amount as i32,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully updated Transfer",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update transfer {transfer_id} - data missing in gRPC response",);
                    HttpError::Internal("Transfer data is missing in gRPC response".into())
                })?;

                let transfer_response: TransferResponse = data.into();

                let api_response = ApiResponse {
                    data: transfer_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    format!("transfer:find_by_id:{}", api_response.data.clone().id),
                    format!(
                        "transfer:find_transfer_from:{}",
                        api_response.data.clone().transfer_from
                    ),
                    format!(
                        "transfer:find_transfer_to:{}",
                        api_response.data.clone().transfer_to
                    ),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let cache_key = vec![
                    format!("transfer:find_by_id:{}", api_response.data.clone().id),
                    format!(
                        "transfer:find_transfer_from:{}",
                        api_response.data.transfer_from
                    ),
                    format!(
                        "transfer:find_transfer_to:{}",
                        api_response.data.transfer_to
                    ),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!(
                    "transfer {transfer_id} updated successfully FROM {masked_from} TO {masked_to}"
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update Transfer")
                    .await;
                error!("update transfer {transfer_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, HttpError> {
        info!("trashing transfer id: {transfer_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("transfer.id", transfer_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully trashed Transfer",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash transfer {transfer_id} - data missing in gRPC response");
                    HttpError::Internal("Transfer data is missing in gRPC response".into())
                })?;

                let cache_keys = vec![
                    format!("transfer:find_by_id:{}", data.id),
                    format!("transfer:find_transfer_from:{}", data.transfer_from),
                    format!("transfer:find_transfer_to:{}", data.transfer_to),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let transfer_response: TransferResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: transfer_response,
                    status: inner.status,
                    message: inner.message,
                };

                info!("transfer {transfer_id} trashed successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash Transfer")
                    .await;
                error!("trash transfer {transfer_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, HttpError> {
        info!("restoring transfer id: {transfer_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreTransfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("transfer.id", transfer_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored Transfer",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore transfer {transfer_id} - data missing in gRPC response");
                    HttpError::Internal("Transfer data is missing in gRPC response".into())
                })?;

                let cache_keys = vec![
                    format!("transfer:find_by_id:{}", data.id),
                    format!("transfer:find_transfer_from:{}", data.transfer_from),
                    format!("transfer:find_transfer_to:{}", data.transfer_to),
                    "transfer:find_by_transfer_from".to_string(),
                    "transfer:find_by_transfer_to".to_string(),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let transfer_response: TransferResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: transfer_response,
                    status: inner.status,
                    message: inner.message,
                };

                info!("transfer {transfer_id} restored successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore Transfer")
                    .await;
                error!("restore transfer {transfer_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting transfer id: {transfer_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteTransferPermanent",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("transfer.id", transfer_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_transfer_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully permanently deleted Transfer",
                )
                .await;
                let inner = response.into_inner();

                let cache_keys = vec![
                    format!("transfer:find_by_id:{}", transfer_id),
                    "transfer:find_transfer_from:*".to_string(),
                    "transfer:find_transfer_to:{}".to_string(),
                    "transfer:find_by_transfer_from".to_string(),
                    "transfer:find_by_transfer_to".to_string(),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                info!("transfer {transfer_id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to permanently delete Transfer",
                )
                .await;
                error!("delete transfer {transfer_id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed transfers");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllTransfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_transfer(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all Transfers",
                )
                .await;
                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "transfer:find_by_id:*".to_string(),
                    "transfer:find_by_transfer_from".to_string(),
                    "transfer:find_by_transfer_to".to_string(),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("all trashed transfers restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all Transfers",
                )
                .await;
                error!("restore all transfers failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all transfers");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllTransfersPermanent",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "delete_all_permanent"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_transfer_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully permanently deleted all Transfers",
                )
                .await;

                let inner = response.into_inner();

                let cache_keys = vec![
                    "transfer:find_by_id:*".to_string(),
                    "transfer:find_by_transfer_from".to_string(),
                    "transfer:find_by_transfer_to".to_string(),
                    "transfer:find_all:*".to_string(),
                    "transfer:find_by_active:*".to_string(),
                    "transfer:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                info!("all transfers permanently deleted");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to permanently delete all Transfers",
                )
                .await;
                error!("delete all transfers permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransferStatsAmountGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, HttpError> {
        info!("fetching monthly transfer AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmounts",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_monthly_amounts"),
                KeyValue::new("transfer.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transfer:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_amounts(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly amounts",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
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
                    "fetched {} monthly transfer amount records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly amounts",
                )
                .await;
                error!("fetch monthly transfer AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, HttpError> {
        info!("fetching yearly transfer AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmounts",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_amounts"),
                KeyValue::new("transfer.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transfer:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_amounts(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly amounts",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
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
                    "fetched {} yearly transfer amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch yearly amounts")
                    .await;
                error!("fetch yearly transfer AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransferStatsStatusGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer SUCCESS status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccess",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_month_status_success"),
                KeyValue::new("transfer.year", req.year.to_string()),
                KeyValue::new("transfer.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransferStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:month_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly success status",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusSuccess> =
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
                    "fetched {} monthly SUCCESS transfer records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly success status",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS transfer status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, HttpError> {
        info!("fetching yearly transfer SUCCESS status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccess",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_status_success"),
                KeyValue::new("transfer.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transfer:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful transfers in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly success status",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS transfer records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly success status",
                )
                .await;
                error!("fetch yearly SUCCESS transfer status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer FAILED status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailed",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_month_status_failed"),
                KeyValue::new("transfer.year", req.year.to_string()),
                KeyValue::new("transfer.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransferStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:month_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly failed status",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED transfer records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly failed status",
                )
                .await;
                error!(
                    "fetch monthly FAILED transfer status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, HttpError> {
        info!("fetching yearly transfer FAILED status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailed",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_status_failed"),
                KeyValue::new("transfer.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("transfer:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed transfers in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly failed status",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED transfer records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly failed status",
                )
                .await;
                error!("fetch yearly FAILED transfer status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransferStatsAmountByCardNumberGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_sender_bycard(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transfer AMOUNT as SENDER for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsSenderByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_monthly_amounts_sender_bycard"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:monthly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (as sender) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_amounts_by_sender_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly amounts as sender by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
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
                    "fetched {} monthly transfer amount records as SENDER for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly amounts as sender by card",
                )
                .await;
                error!(
                    "fetch monthly transfer AMOUNT as SENDER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_receiver_bycard(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transfer AMOUNT as RECEIVER for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsReceiverByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_monthly_amounts_receiver_bycard"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:monthly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (as receiver) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_amounts_by_receiver_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly amounts as receiver by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
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
                    "fetched {} monthly transfer amount records as RECEIVER for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly amounts as receiver by card",
                )
                .await;
                error!(
                    "fetch monthly transfer AMOUNT as RECEIVER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_sender_bycard(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer AMOUNT as SENDER for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsSenderByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_amounts_sender_bycard"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:yearly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (as sender) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_amounts_by_sender_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly amounts as sender by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
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
                    "fetched {} yearly transfer amount records as SENDER for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly amounts as sender by card",
                )
                .await;
                error!(
                    "fetch yearly transfer AMOUNT as SENDER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_receiver_bycard(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer AMOUNT as RECEIVER for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsReceiverByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_amounts_receiver_bycard"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:yearly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (as receiver) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_amounts_by_receiver_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly amounts as receiver by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
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
                    "fetched {} yearly transfer amount records as RECEIVER for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly amounts as receiver by card",
                )
                .await;
                error!(
                    "fetch yearly transfer AMOUNT as RECEIVER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TransferStatsStatusByCardNumberGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_by_card(
        &self,
        req: &DomainMonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccessByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_month_status_success_by_card"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
                KeyValue::new("transfer.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:month_status_success:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful monthly transfers in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful monthly transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly success status by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusSuccess> =
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
                    "fetched {} monthly SUCCESS transfer records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly success status by card",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS transfer status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_by_card(
        &self,
        req: &DomainYearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_status_success_by_card"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:yearly_status_success:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful transfers in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly success status by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS transfer records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly success status by card",
                )
                .await;
                error!(
                    "fetch yearly SUCCESS transfer status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_by_card(
        &self,
        req: &DomainMonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailedByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_month_status_failed_by_card"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
                KeyValue::new("transfer.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:month_status_failed:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed monthly transfers in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed monthly transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly failed status by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED transfer records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly failed status by card",
                )
                .await;
                error!(
                    "fetch monthly FAILED transfer status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_by_card(
        &self,
        req: &DomainYearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedByCard",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "get_yearly_status_failed_by_card"),
                KeyValue::new("transfer.card_number", masked_card.clone()),
                KeyValue::new("transfer.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "transfer:yearly_status_failed:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed transfers in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly failed status by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED transfer records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly failed status by card",
                )
                .await;
                error!(
                    "fetch yearly FAILED transfer status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
