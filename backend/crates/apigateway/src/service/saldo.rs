use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::{
    card::FindByCardNumberRequest,
    saldo::{
        CreateSaldoRequest, FindAllSaldoRequest, FindByIdSaldoRequest,
        FindMonthlySaldoTotalBalance, FindYearlySaldo, UpdateSaldoRequest,
        saldo_service_client::SaldoServiceClient,
    },
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::saldo::http::{
        SaldoBalanceGrpcClientTrait, SaldoCommandGrpcClientTrait, SaldoGrpcClientServiceTrait,
        SaldoQueryGrpcClientTrait, SaldoTotalBalanceGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::saldo::{
            CreateSaldoRequest as DomainCreateSaldoRequest, FindAllSaldos as DomainFindAllSaldos,
            MonthTotalSaldoBalance as DomainMonthTotalSaldoBalance,
            UpdateSaldoRequest as DomainUpdateSaldoRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, SaldoMonthBalanceResponse,
            SaldoMonthTotalBalanceResponse, SaldoResponse, SaldoResponseDeleteAt,
            SaldoYearBalanceResponse, SaldoYearTotalBalanceResponse,
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

pub struct SaldoGrpcClientService {
    client: SaldoServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl SaldoGrpcClientService {
    pub fn new(client: SaldoServiceClient<Channel>, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("saldo-client-service")
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
impl SaldoGrpcClientServiceTrait for SaldoGrpcClientService {}

#[async_trait]
impl SaldoQueryGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;

        info!(
            "fetching all saldos - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllSaldoRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "saldo:find_all:page:{page}:size:{page_size}:search:{}",
            request.search,
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

        match self.client.clone().find_all_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched saldos")
                    .await;

                let inner = response.into_inner();
                let data: Vec<SaldoResponse> = inner.data.into_iter().map(Into::into).collect();

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

                info!("fetched {} saldos", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch saldos")
                    .await;
                error!("fetch all saldos failed: {status:?}");

                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_active(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;

        info!(
            "fetching active saldos - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllSaldoRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "saldo:find_by_active:page:{page}:size:{page_size}:search:{}",
            request.search,
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

        match self.client.clone().find_by_active(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active saldos",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoResponseDeleteAt> =
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

                info!("fetched {} active saldos", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch active saldos")
                    .await;
                error!("fetch active saldos failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_trashed(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, HttpError> {
        let page = request.page;
        let page_size = request.page;

        info!(
            "fetching trashed saldos - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllSaldoRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "saldo:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            request.search,
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

        match self.client.clone().find_by_trashed(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed saldos",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoResponseDeleteAt> =
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

                info!("fetched {} trashed saldos", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch trashed saldos")
                    .await;
                error!("fetch trashed saldos failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, HttpError> {
        info!("fetching saldo by id: {id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindSaldoById",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("saldo_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdSaldoRequest { saldo_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self.client.clone().find_by_id_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched saldo by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("saldo {id} - data missing in gRPC response");
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let data: SaldoResponse = data.into();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found saldo {id}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch saldo by id")
                    .await;
                error!("find saldo {id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError> {
        let masked_card = mask_card_number(card_number);

        info!("fetching saldo by card_number: {masked_card}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindSaldoByCard",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "find_by_card"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberRequest {
            card_number: card_number.to_string(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

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

        match self.client.clone().find_by_card_number(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched saldo by card number",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("saldo with card_number {card_number} - data missing in gRPC response");
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let data: SaldoResponse = data.into();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("found saldo with card_number {card_number}");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch saldo by card number",
                )
                .await;
                error!("find saldo with card_number {card_number} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl SaldoCommandGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: &DomainCreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError> {
        let masked_card = mask_card_number(&request.card_number);

        info!(
            "creating saldo for card: {masked_card} with balance: {}",
            request.total_balance
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "create"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(CreateSaldoRequest {
            card_number: request.card_number.clone(),
            total_balance: request.total_balance as i32,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully created saldo")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("saldo creation failed - data missing in gRPC response for card: {masked_card}");
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let saldo_response: SaldoResponse = data.into();

                let api_response = ApiResponse {
                    data: saldo_response,
                    message: inner.message,
                    status: inner.status,
                };

                let cache_key_delete = vec![
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_key_delete {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let cache_key = [
                    format!("saldo:find_by_card:card_number:{}", masked_card),
                    format!("saldo:find_by_id:id:{}", api_response.data.clone().id),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!("saldo created successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create saldo")
                    .await;
                error!("create saldo for card {masked_card} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: &DomainUpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError> {
        let masked_card = mask_card_number(&request.card_number);
        let saldo_id = request
            .saldo_id
            .ok_or_else(|| HttpError::Internal("saldo_id is required".to_string()))?;

        info!(
            "updating saldo id: {saldo_id} for card: {} with new balance: {}",
            masked_card, request.total_balance
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "update"),
                KeyValue::new("saldo_id", saldo_id.to_string()),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut grpc_req = Request::new(UpdateSaldoRequest {
            saldo_id,
            card_number: request.card_number.clone(),
            total_balance: request.total_balance as i32,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully updated saldo")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update saldo {saldo_id} - data missing in gRPC response",);
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let saldo_response: SaldoResponse = data.into();

                let api_response = ApiResponse {
                    data: saldo_response,
                    message: inner.message,
                    status: inner.status,
                };

                let cache_key_delete = vec![
                    format!("saldo:find_by_id:id:{}", api_response.data.clone().id),
                    format!("saldo:find_by_card:card_number:{}", masked_card),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_key_delete {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let cache_key = [
                    format!("saldo:find_by_card:card_number:{}", masked_card),
                    format!("saldo:find_by_id:id:{}", api_response.data.id),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!("saldo {saldo_id} updated successfully for card: {masked_card}");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update saldo")
                    .await;
                error!("update saldo {saldo_id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, HttpError> {
        info!("trashing saldo id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("saldo_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdSaldoRequest { saldo_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully trashed saldo")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash saldo {id} - data missing in gRPC response");
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let saldo_response: SaldoResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: saldo_response,
                    message: inner.message,
                    status: inner.status,
                };

                let cache_key_delete = vec![
                    format!("saldo:find_by_id:id:{id}"),
                    "saldo:find_by_card:card_number:*".to_string(),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_key_delete {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("saldo {id} trashed successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash saldo")
                    .await;
                error!("trash saldo {id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, HttpError> {
        info!("restoring saldo id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("saldo_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdSaldoRequest { saldo_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully restored saldo")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore saldo {id} - data missing in gRPC response");
                    HttpError::Internal("Saldo data is missing in gRPC response".into())
                })?;

                let saldo_response: SaldoResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: saldo_response,
                    message: inner.message,
                    status: inner.status,
                };

                let cache_keys = vec![
                    format!("saldo:find_by_id:id:{id}"),
                    "saldo:find_by_card:card_number:*".to_string(),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("saldo {id} restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore saldo")
                    .await;
                error!("restore saldo {id} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting saldo id: {id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteSaldoPermanent",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("saldo_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdSaldoRequest { saldo_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().delete_saldo_permanent(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted saldo permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("saldo:find_by_id:id:{id}"),
                    "saldo:find_by_card:card_number:*".to_string(),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("saldo {id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete saldo permanently",
                )
                .await;
                error!("delete saldo {id} permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed saldos");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllSaldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_saldo(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all trashed saldos",
                )
                .await;

                let inner = response.into_inner();
                info!("all trashed saldos restored successfully");

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "saldo:find_by_id:id:*".to_string(),
                    "saldo:find_by_card:card_number:*".to_string(),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all trashed saldos",
                )
                .await;
                error!("restore all saldos failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all saldos");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllSaldosPermanent",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_saldo_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted all saldos permanently",
                )
                .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "saldo:find_by_id:id:*".to_string(),
                    "saldo:find_by_card:card_number:*".to_string(),
                    "saldo:find_all:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all saldos permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all saldos permanently",
                )
                .await;
                error!("delete all saldos permanently failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl SaldoBalanceGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoMonthBalanceResponse>>, HttpError> {
        info!("fetching monthly BALANCE for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthBalanceSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "get_month_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearlySaldo { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("saldo:monthly_balance:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoMonthBalanceResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly balance in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_saldo_balances(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoMonthBalanceResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    status: inner.status,
                    message: inner.message,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} monthly balance records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly balance",
                )
                .await;
                error!("fetch monthly BALANCE for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearBalanceResponse>>, HttpError> {
        info!("fetching yearly BALANCE for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearBalanceSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "get_year_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearlySaldo { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("saldo:yearly_balance:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoYearBalanceResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly balance in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_saldo_balances(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoYearBalanceResponse> =
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
                    "fetched {} yearly balance records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch yearly balance")
                    .await;
                error!("fetch yearly BALANCE for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}

#[async_trait]
impl SaldoTotalBalanceGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_total_balance(
        &self,
        req: &DomainMonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, HttpError> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly TOTAL BALANCE for {month_str} {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthTotalBalanceSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "get_month_total_balance"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindMonthlySaldoTotalBalance {
            year: req.year,
            month: req.month,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("saldo:monthly_total_balance:year:{}", req.year);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total balance in cache for year: {}",
                req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly total balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_total_saldo_balance(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly total balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoMonthTotalBalanceResponse> =
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
                    "fetched {} monthly total balance records for {month_str} {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly total balance",
                )
                .await;
                error!(
                    "fetch monthly TOTAL BALANCE for {month_str} {} failed: {status:?}",
                    req.year
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, HttpError> {
        info!("fetching yearly TOTAL BALANCE for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearTotalBalanceSaldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "get_year_total_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearlySaldo { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("saldo:yearly_total_balance:year:{}", year);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly total balance in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly total balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_year_total_saldo_balance(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly total balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<SaldoYearTotalBalanceResponse> =
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
                    "fetched {} yearly total balance records for year {year}",
                    api_response.data.len()
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly total balance",
                )
                .await;
                error!("fetch yearly TOTAL BALANCE for year {year} failed: {status:?}");
                return Err(AppErrorGrpc::from(status).into());
            }
        }
    }
}
