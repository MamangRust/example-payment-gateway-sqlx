use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::withdraw::{
    CreateWithdrawRequest, FindAllWithdrawByCardNumberRequest, FindAllWithdrawRequest,
    FindByIdWithdrawRequest, FindMonthlyWithdrawStatus, FindMonthlyWithdrawStatusCardNumber,
    FindYearWithdrawCardNumber, FindYearWithdrawStatus, FindYearWithdrawStatusCardNumber,
    UpdateWithdrawRequest, withdraw_service_client::WithdrawServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::withdraw::http::{
        WithdrawCommandGrpcClientTrait, WithdrawGrpcClientServiceTrait,
        WithdrawQueryGrpcClientTrait, WithdrawStatsAmountByCardNumberGrpcClientTrait,
        WithdrawStatsAmountGrpcClientTrait, WithdrawStatsStatusByCardNumberGrpcClientTrait,
        WithdrawStatsStatusGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::withdraw::{
            CreateWithdrawRequest as DomainCreateWithdrawRequest,
            FindAllWithdrawCardNumber as DomainFindAllWithdrawCardNumber,
            FindAllWithdraws as DomainFindAllWithdraws,
            MonthStatusWithdraw as DomainMonthStatusWithdraw,
            MonthStatusWithdrawCardNumber as DomainMonthStatusWithdrawCardNumber,
            UpdateWithdrawRequest as DomainUpdateWithdrawRequest,
            YearMonthCardNumber as DomainYearMonthCardNumber,
            YearStatusWithdrawCardNumber as DomainYearStatusWithdrawCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawMonthlyAmountResponse, WithdrawResponse,
            WithdrawResponseDeleteAt, WithdrawResponseMonthStatusFailed,
            WithdrawResponseMonthStatusSuccess, WithdrawResponseYearStatusFailed,
            WithdrawResponseYearStatusSuccess, WithdrawYearlyAmountResponse,
        },
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_card_number,
        month_name, naive_datetime_to_timestamp,
    },
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct WithdrawGrpcClientService {
    client: WithdrawServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl WithdrawGrpcClientService {
    pub fn new(
        client: WithdrawServiceClient<Channel>,
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
        global::tracer("withdraw-client-service")
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
impl WithdrawGrpcClientServiceTrait for WithdrawGrpcClientService {}

#[async_trait]
impl WithdrawQueryGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching all withdraws - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("withdraw.page", page.to_string()),
                KeyValue::new("withdraw.page_size", page_size.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllWithdrawRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:find_all:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_all_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched all withdraws",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawResponse> = inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                info!("fetched {} withdraws", api_response.data.len());
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch all withdraws")
                    .await;
                error!("fetch all withdraws failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &DomainFindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching withdraws for card: {} - page: {page}, page_size: {page_size}, search: {:?}",
            masked_card, req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllWithdrawsByCardNumber",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "find_all_by_card_number"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.page", page.to_string()),
                KeyValue::new("withdraw.page_size", page_size.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllWithdrawByCardNumberRequest {
            card_number: req.card_number.clone(),
            search: req.search.clone(),
            page,
            page_size,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:find_all_by_card:card:{}:page:{page}:size:{page_size}:search:{}",
            req.card_number,
            req.search.clone()
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

        match self
            .client
            .clone()
            .find_all_withdraw_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched withdraws by card number",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawResponse> = inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} withdraws for card {}",
                    api_response.data.len(),
                    masked_card
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch withdraws by card number",
                )
                .await;
                error!(
                    "fetch withdraws for card {} failed: {status:?}",
                    masked_card
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError> {
        info!("fetching withdraw by id: {withdraw_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindWithdrawById",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("withdraw.id", withdraw_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self.client.clone().find_by_id_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully found withdraw by id",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("withdraw {withdraw_id} - data missing in gRPC response");
                    HttpError::Internal("Withdraw data is missing in gRPC response".into())
                })?;

                let api_response = ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found withdraw {withdraw_id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to find withdraw by id")
                    .await;
                error!("find withdraw {withdraw_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching active withdraws - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "find_by_active"),
                KeyValue::new("withdraw.page", page.to_string()),
                KeyValue::new("withdraw.page_size", page_size.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllWithdrawRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:find_by_active:page:{page}:size:{page_size}:search:{}",
            req.search.clone()
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

        match self.client.clone().find_by_active(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active withdraws",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseDeleteAt> =
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

                info!("fetched {} active withdraws", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch active withdraws",
                )
                .await;
                error!("fetch active withdraws failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching trashed withdraws - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "find_by_trashed"),
                KeyValue::new("withdraw.page", page.to_string()),
                KeyValue::new("withdraw.page_size", page_size.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllWithdrawRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            req.search.clone()
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

        match self.client.clone().find_by_trashed(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed withdraws",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseDeleteAt> =
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

                info!("fetched {} trashed withdraws", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch trashed withdraws",
                )
                .await;
                error!("fetch trashed withdraws failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl WithdrawCommandGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating withdraw for card: {masked_card}, amount: {}",
            req.withdraw_amount
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "create"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.amount", req.withdraw_amount.to_string()),
            ],
        );

        let date = naive_datetime_to_timestamp(req.withdraw_time);

        let mut grpc_req = Request::new(CreateWithdrawRequest {
            card_number: req.card_number.clone(),
            withdraw_amount: req.withdraw_amount,
            withdraw_time: Some(date),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully created Withdraw",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("withdraw creation failed - data missing in gRPC response for card: {masked_card}");
                    HttpError::Internal("Withdraw data is missing in gRPC response".into())
                })?;

                let withdraw_response: WithdrawResponse = data.into();

                let api_response = ApiResponse {
                    data: withdraw_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_delete_keys = vec![
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_delete_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let cache_key = format!("withdraw:find_by_id:{}", api_response.data.id);

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("withdraw created successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create Withdraw")
                    .await;
                error!("create withdraw for card {masked_card} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);

        let withdraw_id = req
            .withdraw_id
            .ok_or_else(|| HttpError::Internal("widhdraw_id is required".to_string()))?;

        info!(
            "updating withdraw id: {withdraw_id} for card: {masked_card}, new amount: {}",
            req.withdraw_amount
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "update"),
                KeyValue::new("withdraw.id", withdraw_id.to_string()),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.amount", req.withdraw_amount.to_string()),
            ],
        );

        let date = naive_datetime_to_timestamp(req.withdraw_time);

        let mut grpc_req = Request::new(UpdateWithdrawRequest {
            card_number: req.card_number.clone(),
            withdraw_id,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: Some(date),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully updated Withdraw",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "update withdraw {} - data missing in gRPC response",
                        withdraw_id
                    );
                    HttpError::Internal("Withdraw data is missing in gRPC response".into())
                })?;

                let withdraw_response: WithdrawResponse = data.into();

                let api_response = ApiResponse {
                    data: withdraw_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("withdraw:find_by_id:{}", api_response.data.clone().id),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                let cache_key = format!("withdraw:find_by_id:{}", api_response.data.id);

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("withdraw {withdraw_id} updated successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update Withdraw")
                    .await;
                error!("update withdraw {withdraw_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, HttpError> {
        info!("trashing withdraw id: {withdraw_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashedWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("withdraw.id", withdraw_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully trashed Withdraw",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash withdraw {withdraw_id} - data missing in gRPC response");
                    HttpError::Internal("Withdraw data is missing in gRPC response".into())
                })?;

                let withdraw_response: WithdrawResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: withdraw_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("withdraw:find_by_id:{}", api_response.data.clone().id),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("withdraw {withdraw_id} trashed successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash Withdraw")
                    .await;
                error!("trash withdraw {withdraw_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, HttpError> {
        info!("restoring withdraw id: {withdraw_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("withdraw.id", withdraw_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored Withdraw",
                )
                .await;
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore withdraw {withdraw_id} - data missing in gRPC response");
                    HttpError::Internal("Withdraw data is missing in gRPC response".into())
                })?;

                let withdraw_response: WithdrawResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: withdraw_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("withdraw:find_by_id:{}", api_response.data.clone().id),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("withdraw {withdraw_id} restored successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore Withdraw")
                    .await;
                error!("restore withdraw {withdraw_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting withdraw id: {withdraw_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeletePermanentWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("withdraw.id", withdraw_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_withdraw_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully permanently deleted Withdraw",
                )
                .await;
                let inner = response.into_inner();

                let cache_keys = vec![
                    "withdraw:find_by_id:*".to_string(),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("withdraw {withdraw_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to permanently delete Withdraw",
                )
                .await;
                error!("delete withdraw {withdraw_id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed withdraws");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_withdraw(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all Withdraws",
                )
                .await;
                let inner = response.into_inner();

                let cache_keys = vec![
                    "withdraw:find_by_card:*".to_string(),
                    "withdraw:find_by_id:*".to_string(),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("all trashed withdraws restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all Withdraws",
                )
                .await;
                error!("restore all withdraws failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all withdraws");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_withdraw_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully permanently deleted all Withdraws",
                )
                .await;
                let inner = response.into_inner();

                let cache_keys = vec![
                    "withdraw:find_by_id:*".to_string(),
                    "withdraw:find_all:*".to_string(),
                    "withdraw:find_by_active:*".to_string(),
                    "withdraw:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                    info!("Invalidated cache key: {}", key);
                }

                info!("all withdraws permanently deleted");

                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to permanently delete all Withdraws",
                )
                .await;
                error!("delete all withdraws permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsAmountGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, HttpError> {
        info!("fetching monthly withdraw AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_monthly_withdraws"),
                KeyValue::new("withdraw.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("withdrawal:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly withdrawal amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly withdrawal amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().find_monthly_withdraws(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly withdraws",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawMonthlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly withdraw amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly withdraws",
                )
                .await;
                error!("fetch monthly withdraw AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, HttpError> {
        info!("fetching yearly withdraw AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyWithdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_withdraws"),
                KeyValue::new("withdraw.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("withdrawal:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly withdrawal amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly withdrawal amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().find_yearly_withdraws(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly withdraws",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} yearly withdraw amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly withdraws",
                )
                .await;
                error!("fetch yearly withdraw AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsStatusGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw SUCCESS status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccessWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_month_status_success"),
                KeyValue::new("withdraw.year", req.year.to_string()),
                KeyValue::new("withdraw.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyWithdrawStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:month_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful withdrawals in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraw_status_success(grpc_req)
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
                let data: Vec<WithdrawResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} monthly SUCCESS withdraw records for {month_str} {}",
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
                    "fetch monthly SUCCESS withdraw status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, HttpError> {
        info!("fetching yearly withdraw SUCCESS status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_status_success"),
                KeyValue::new("withdraw.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("withdrawal:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful withdrawals in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraw_status_success(grpc_req)
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
                let data: Vec<WithdrawResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS withdraw records for year {year}",
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
                error!("fetch yearly SUCCESS withdraw status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw FAILED status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailedWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_month_status_failed"),
                KeyValue::new("withdraw.year", req.year.to_string()),
                KeyValue::new("withdraw.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyWithdrawStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:month_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed withdrawals in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraw_status_failed(grpc_req)
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
                let data: Vec<WithdrawResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED withdraw records for {month_str} {}",
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
                    "fetch monthly FAILED withdraw status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, HttpError> {
        info!("fetching yearly withdraw FAILED status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedWithdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_status_failed"),
                KeyValue::new("withdraw.year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("withdrawal:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed withdrawals in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraw_status_failed(grpc_req)
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
                let data: Vec<WithdrawResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED withdraw records for year {year}",
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
                error!("fetch yearly FAILED withdraw status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsAmountByCardNumberGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_bycard(
        &self,
        req: &DomainYearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly withdraw AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyWithdrawsByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_monthly_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:monthly_by_card:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly withdrawal amounts in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly withdrawal amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraws_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly withdraws by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawMonthlyAmountResponse> =
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
                    "fetched {} monthly withdraw amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly withdraws by card",
                )
                .await;
                error!(
                    "fetch monthly withdraw AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_bycard(
        &self,
        req: &DomainYearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyWithdrawsByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:yearly_by_card:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly withdrawal amounts in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly withdrawal amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraws_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly withdraws by card",
                )
                .await;
                let inner = response.into_inner();
                let data: Vec<WithdrawYearlyAmountResponse> =
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
                    "fetched {} yearly withdraw amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly withdraws by card",
                )
                .await;
                error!(
                    "fetch yearly withdraw AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsStatusByCardNumberGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusSuccessByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_month_status_success_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
                KeyValue::new("withdraw.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:month_status_success:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful monthly withdrawals in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful monthly withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraw_status_success_card_number(grpc_req)
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
                let data: Vec<WithdrawResponseMonthStatusSuccess> =
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
                    "fetched {} monthly SUCCESS withdraw records for card {masked_card} {month_str} {}",
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
                    "fetch monthly SUCCESS withdraw status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_status_success_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:yearly_status_success:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful withdrawals in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraw_status_success_card_number(grpc_req)
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
                let data: Vec<WithdrawResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS withdraw records for card {masked_card} year {}",
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
                    "fetch yearly SUCCESS withdraw status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthStatusFailedByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_month_status_failed_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
                KeyValue::new("withdraw.month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:month_status_failed:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed monthly withdrawals in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed monthly withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraw_status_failed_card_number(grpc_req)
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
                let data: Vec<WithdrawResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED withdraw records for card {masked_card} {month_str} {}",
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
                    "fetch monthly FAILED withdraw status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedByCard",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "get_yearly_status_failed_bycard"),
                KeyValue::new("withdraw.card_number", masked_card.clone()),
                KeyValue::new("withdraw.year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "withdrawal:yearly_status_failed:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed withdrawals in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed withdrawals retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraw_status_failed_card_number(grpc_req)
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
                let data: Vec<WithdrawResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED withdraw records for card {masked_card} year {}",
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
                    "fetch yearly FAILED withdraw status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
