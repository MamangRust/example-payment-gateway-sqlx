use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::topup::{
    CreateTopupRequest, FindAllTopupByCardNumberRequest, FindAllTopupRequest,
    FindByCardNumberTopupRequest, FindByIdTopupRequest, FindMonthlyTopupStatus,
    FindMonthlyTopupStatusCardNumber, FindYearTopupCardNumber, FindYearTopupStatus,
    FindYearTopupStatusCardNumber, UpdateTopupRequest, topup_service_client::TopupServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::topup::http::{
        TopupCommandGrpcClientTrait, TopupGrpcClientServiceTrait, TopupQueryGrpcClientTrait,
        TopupStatsAmountByCardNumberGrpcClientTrait, TopupStatsAmountGrpcClientTrait,
        TopupStatsMethodByCardNumberGrpcClientTrait, TopupStatsMethodGrpcClientTrait,
        TopupStatsStatusByCardNumberGrpcClientTrait, TopupStatsStatusGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::topup::{
            CreateTopupRequest as DomainCreateTopupRequest, FindAllTopups as DomainFindAllTopups,
            FindAllTopupsByCardNumber as DomainFindAllTopupsByCardNumber,
            MonthTopupStatus as DomainMonthTopupStatus,
            MonthTopupStatusCardNumber as DomainMonthTopupStatusCardNumber,
            UpdateTopupRequest as DomainUpdateTopupRequest,
            YearMonthMethod as DomainYearMonthMethod,
            YearTopupStatusCardNumber as DomainYearTopupStatusCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TopupMonthAmountResponse, TopupMonthMethodResponse,
            TopupResponse, TopupResponseDeleteAt, TopupResponseMonthStatusFailed,
            TopupResponseMonthStatusSuccess, TopupResponseYearStatusFailed,
            TopupResponseYearStatusSuccess, TopupYearlyAmountResponse, TopupYearlyMethodResponse,
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

pub struct TopupGrpcClientService {
    client: TopupServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl TopupGrpcClientService {
    pub fn new(client: TopupServiceClient<Channel>, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("topup-client-service")
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
impl TopupGrpcClientServiceTrait for TopupGrpcClientService {}

#[async_trait]
impl TopupQueryGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching all topups - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTopupRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:find_all:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_all_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched topups")
                    .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

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

                info!("fetched {} topups", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch topups")
                    .await;
                error!("fetch all topups failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &DomainFindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);

        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching topups for card: {} - page: {page}, page_size: {page_size}, search: {:?}",
            masked_card, req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllTopupByCardNumber",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_all_by_card_number"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTopupByCardNumberRequest {
            card_number: req.card_number.clone(),
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:find_by_card_number:card:{}:page:{page}:size:{page_size}:search:{}",
            masked_card, req.search
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

        match self
            .client
            .clone()
            .find_all_topup_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched topups by card number",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

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
                    "fetched {} topups for card {}",
                    api_response.data.len(),
                    masked_card
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch topups by card number",
                )
                .await;
                error!("fetch topups for card {} failed: {status:?}", masked_card);
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_active(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching active topups - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTopupRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:find_by_active:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_by_active(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active topups",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseDeleteAt> =
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

                info!("fetched {} active topups", api_response.data.len());
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch active topups")
                    .await;
                error!("fetch active topups failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_trashed(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;

        info!(
            "fetching trashed topups - page: {page}, page_size: {page_size}, search: {:?}",
            req.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", req.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllTopupRequest {
            page,
            page_size,
            search: req.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            req.search
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

        match self.client.clone().find_by_trashed(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed topups",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseDeleteAt> =
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

                info!("fetched {} trashed topups", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch trashed topups")
                    .await;
                error!("fetch trashed topups failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, HttpError> {
        let masked_card = mask_card_number(card_number);
        info!("fetching topups by card: {masked_card}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTopupByCard",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_by_card"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberTopupRequest {
            card_number: card_number.to_string(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self
            .client
            .clone()
            .find_by_card_number_topup(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched topups by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    message: inner.message,
                    status: inner.status,
                    data,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} topups for card {masked_card}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch topups by card")
                    .await;
                error!("fetch topups by card {masked_card} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, HttpError> {
        info!("fetching topup by id: {topup_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTopupById",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self.client.clone().find_by_id_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched topup by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("topup {topup_id} - data missing in gRPC response");
                    HttpError::Internal("Topup data is missing in gRPC response".into())
                })?;

                let topup_response: TopupResponse = data.into();

                let api_response = ApiResponse {
                    data: topup_response,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found topup {topup_id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch topup by id")
                    .await;
                error!("find topup {topup_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupCommandGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating topup for card: {masked_card}, amount: {}, method: {}",
            req.topup_amount, req.topup_method
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "create"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(CreateTopupRequest {
            card_number: req.card_number.clone(),
            topup_amount: req.topup_amount as i32,
            topup_method: req.topup_method.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully created topup")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("topup creation failed - data missing in gRPC response for card: {masked_card}");
                    HttpError::Internal("Topup data is missing in gRPC response".into())
                })?;

                let topup_response: TopupResponse = data.into();

                let api_response = ApiResponse {
                    data: topup_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("topup:find_all_by_card_number:card:{}:*", masked_card),
                    format!("topup:find_by_card:card_number:{}", masked_card),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let cache_key = format!("card:find_by_id:{}", api_response.data.id);
                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("topup created successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create topup")
                    .await;
                error!("create topup for card {masked_card} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);

        let topup_id = req
            .topup_id
            .ok_or_else(|| HttpError::Internal("topup_id is required".to_string()))?;

        info!(
            "updating topup id: {topup_id} for card: {}, new amount: {}, method: {}",
            masked_card, req.topup_amount, req.topup_method
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "update"),
                KeyValue::new("topup_id", topup_id.to_string()),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(UpdateTopupRequest {
            card_number: req.card_number.clone(),
            topup_id,
            topup_amount: req.topup_amount as i32,
            topup_method: req.topup_method.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully updated topup")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update topup {topup_id} - data missing in gRPC response",);
                    HttpError::Internal("Topup data is missing in gRPC response".into())
                })?;

                let topup_response: TopupResponse = data.into();

                let api_response = ApiResponse {
                    data: topup_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let cache_key = format!("topup:find_by_id:{}", api_response.data.id);

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("topup {topup_id} updated successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update topup")
                    .await;
                error!("update topup {topup_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, HttpError> {
        info!("trashing topup id: {topup_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully trashed topup")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash topup {topup_id} - data missing in gRPC response");
                    HttpError::Internal("Topup data is missing in gRPC response".into())
                })?;

                let topup_response: TopupResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: topup_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("topup {topup_id} trashed successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash topup")
                    .await;
                error!("trash topup {topup_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, HttpError> {
        info!("restoring topup id: {topup_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully restored topup")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore topup {topup_id} - data missing in gRPC response");
                    HttpError::Internal("Topup data is missing in gRPC response".into())
                })?;

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_id:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let topup_response: TopupResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: topup_response,
                    status: inner.status,
                    message: inner.message,
                };

                info!("topup {topup_id} restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore topup")
                    .await;
                error!("restore topup {topup_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting topup id: {topup_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteTopupPermanent",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().delete_topup_permanent(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted topup permanently",
                )
                .await;

                let inner = response.into_inner();
                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_id:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("topup {topup_id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete topup permanently",
                )
                .await;
                error!("delete topup {topup_id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed topups");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllTopups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_topup(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all trashed topups",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_id:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all trashed topups restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all trashed topups",
                )
                .await;
                error!("restore all topups failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_permanent(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all topups");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllTopupsPermanent",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "delete_all_permanent"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_topup_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted all topups permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "topup:find_all_by_card_number:card:*:*".to_string(),
                    "topup:find_by_id:*".to_string(),
                    "topup:find_by_card:card_number:*".to_string(),
                    "topup:find_by_active:*".to_string(),
                    "topup:find_by_trashed:*".to_string(),
                    "topup:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all topups permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all topups permanently",
                )
                .await;
                error!("delete all topups permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsAmountGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, HttpError> {
        info!("fetching monthly topup AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly top-up amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly top-up amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_amounts(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup amounts",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupMonthAmountResponse> =
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
                    "fetched {} monthly topup amount records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup amounts",
                )
                .await;
                error!("fetch monthly topup AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, HttpError> {
        info!("fetching yearly topup AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly top-up amounts in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly top-up amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_amounts(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup amounts",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupYearlyAmountResponse> =
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
                    "fetched {} yearly topup amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup amounts",
                )
                .await;
                error!("fetch yearly topup AMOUNT for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsMethodGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, HttpError> {
        info!("fetching monthly topup METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyMethodsTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_monthly_methods"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:monthly_methods:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly top-up methods in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly top-up methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_methods(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup methods",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupMonthMethodResponse> =
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
                    "fetched {} monthly topup method records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup methods",
                )
                .await;
                error!("fetch monthly topup METHOD for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, HttpError> {
        info!("fetching yearly topup METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyMethodsTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_methods"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:yearly_methods:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly top-up methods in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly top-up methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_methods(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup methods",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupYearlyMethodResponse> =
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
                    "fetched {} yearly topup method records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup methods",
                )
                .await;
                error!("fetch yearly topup METHOD for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsStatusGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup SUCCESS status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyStatusSuccessTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_month_status_success"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTopupStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_status_success:month:{}:year:{}",
            req.month, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful top-ups in cache for month: {}, year: {}",
                req.month, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup success status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusSuccess> =
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
                    "fetched {} monthly SUCCESS topup records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup success status",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS topup status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, HttpError> {
        info!("fetching yearly topup SUCCESS status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful top-ups in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup success status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS topup records for year {year}",
                    api_response.data.len(),
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup success status",
                )
                .await;
                error!("fetch yearly SUCCESS topup status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup FAILED status for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyStatusFailedTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_month_status_failed"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTopupStatus {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_status_failed:month:{}:year:{}",
            req.month, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed top-ups in cache for month: {}, year: {}",
                req.month, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup failed status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED topup records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup failed status",
                )
                .await;
                error!(
                    "fetch monthly FAILED topup status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, HttpError> {
        info!("fetching yearly topup FAILED status for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatus { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("topup:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed top-ups in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup failed status",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED topup records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup failed status",
                )
                .await;
                error!("fetch yearly FAILED topup status for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsAmountByCardNumberGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly topup AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountsByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_monthly_amounts_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up amounts in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly top-up amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup amounts by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupMonthAmountResponse> =
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
                    "fetched {} monthly topup amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup amounts by card",
                )
                .await;
                error!(
                    "fetch monthly topup AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountsByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_amounts_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:yearly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up amounts in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly top-up amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup amounts by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupYearlyAmountResponse> =
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
                    "fetched {} yearly topup amount records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup amounts by card",
                )
                .await;
                error!(
                    "fetch yearly topup AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsMethodByCardNumberGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_methods_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly topup METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyMethodsByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_monthly_methods_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up methods in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly top-up methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup methods by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupMonthMethodResponse> =
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
                    "fetched {} monthly topup method records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup methods by card",
                )
                .await;
                error!(
                    "fetch monthly topup METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_methods_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyMethodsByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_methods_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:yearly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up methods in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly top-up methods retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup methods by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupYearlyMethodResponse> =
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
                    "fetched {} yearly topup method records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup methods by card",
                )
                .await;
                error!(
                    "fetch yearly topup METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl TopupStatsStatusByCardNumberGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyStatusSuccessByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_month_status_success_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_status_success:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful top-ups in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup success status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusSuccess> =
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
                    "fetched {} monthly SUCCESS topup records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup success status by card",
                )
                .await;
                error!(
                    "fetch monthly SUCCESS topup status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusSuccessByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_status_success_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:yearly_status_success:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful top-ups in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup success status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusSuccess> =
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
                    "fetched {} yearly SUCCESS topup records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup success status by card",
                )
                .await;
                error!(
                    "fetch yearly SUCCESS topup status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyStatusFailedByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_month_status_failed_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlyTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:monthly_status_failed:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found failed top-ups in cache for card: {}", masked_card);
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_topup_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup failed status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusFailed> =
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
                    "fetched {} monthly FAILED topup records for card {masked_card} {month_str} {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup failed status by card",
                )
                .await;
                error!(
                    "fetch monthly FAILED topup status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, HttpError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyStatusFailedByCardTopup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "get_yearly_status_failed_bycard"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "topup:yearly_status_failed:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed top-ups in cache for card: {}",
                masked_card
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed top-ups retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_topup_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup failed status by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusFailed> =
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
                    "fetched {} yearly FAILED topup records for card {masked_card} year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup failed status by card",
                )
                .await;
                error!(
                    "fetch yearly FAILED topup status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
