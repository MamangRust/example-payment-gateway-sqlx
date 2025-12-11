use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::card::{
    CreateCardRequest, FindAllCardRequest, FindByCardNumberRequest, FindByIdCardRequest,
    FindByUserIdCardRequest, FindYearAmount, FindYearAmountCardNumber, FindYearBalance,
    FindYearBalanceCardNumber, UpdateCardRequest, card_service_client::CardServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::card::http::{
        CardCommandGrpcClientTrait, CardDashboardGrpcClientTrait, CardGrpcClientServiceTrait,
        CardQueryGrpcClientTrait, CardStatsBalanceByCardGrpcClientTrait,
        CardStatsBalanceGrpcClientTrait, CardStatsTopupByCardGrpcClientTrait,
        CardStatsTopupGrpcClientTrait, CardStatsTransactionByCardGrpcClientTrait,
        CardStatsTransactionGrpcClientTrait, CardStatsTransferByCardGrpcClientTrait,
        CardStatsTransferGrpcClientTrait, CardStatsWithdrawByCardGrpcClientTrait,
        CardStatsWithdrawGrpcClientTrait,
    },
    cache::CacheStore,
    domain::{
        requests::card::{
            CreateCardRequest as DomainCreateCardRequest, FindAllCards as DomainFindAllCardRequest,
            MonthYearCardNumberCard as DomainMonthYearCardNumberCard,
            UpdateCardRequest as DomainUpdateCardRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt,
            CardResponseMonthAmount, CardResponseMonthBalance, CardResponseYearAmount,
            CardResponseYearlyBalance, DashboardCard, DashboardCardCardNumber,
        },
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_card_number,
        naive_date_to_timestamp,
    },
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct CardGrpcClientService {
    client: CardServiceClient<Channel>,
    metrics: Metrics,
    cache_store: Arc<CacheStore>,
}

impl CardGrpcClientService {
    pub fn new(client: CardServiceClient<Channel>, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            client,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("card-client-service")
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
impl CardGrpcClientServiceTrait for CardGrpcClientService {}

#[async_trait]
impl CardDashboardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip_all)]
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, HttpError> {
        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "GetDashboard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_dashboard"),
            ],
        );

        let mut request = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = "dashboard:global".to_string();

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<DashboardCard>>(&cache_key)
            .await
        {
            info!("✅ Found global dashboard in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Global dashboard retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().dashboard_card(request).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched dashboard data",
                )
                .await;

                let inner = response.into_inner();

                let dashboard_data = inner.data.ok_or_else(|| {
                    error!("Dashboard Card data is missing in gRPC response");

                    HttpError::Internal("Dashboard Card data is missing in gRPC response".into())
                })?;

                let domain_dashboard: DashboardCard = dashboard_data.into();

                let api_response = ApiResponse {
                    status: inner.status,
                    message: inner.message,
                    data: domain_dashboard,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch dashboard data")
                    .await;
                error!("gRPC error: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_number = card_number))]
    async fn get_dashboard_bycard(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, HttpError> {
        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "GetDashboardByCardNumber",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_dashboard_by_card"),
                KeyValue::new("card_number", card_number.clone()),
            ],
        );

        let mut request = Request::new(FindByCardNumberRequest {
            card_number: card_number.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("dashboard:card:{}", mask_card_number(&card_number));

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<DashboardCardCardNumber>>(&cache_key)
            .await
        {
            info!("✅ Found global dashboard in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Global dashboard retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self.client.clone().dashboard_card_number(request).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched dashboard data by card number",
                )
                .await;

                let inner = response.into_inner();

                let dashboard_data = inner.data.ok_or_else(|| {
                    error!("card {card_number} - missing data in gRPC response");

                    HttpError::Internal("Dashboard Card data is missing in gRPC response".into())
                })?;

                let domain_dashboard: DashboardCardCardNumber = dashboard_data.into();

                let api_response = ApiResponse {
                    status: inner.status,
                    message: inner.message,
                    data: domain_dashboard,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch dashboard data by card number",
                )
                .await;
                error!("card {card_number} - gRPC failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardQueryGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;
        let search = &req.search;

        info!(
            "fetching cards - page: {page}, page_size: {page_size}, search: {:?}",
            search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindAllCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllCardRequest {
            page,
            page_size,
            search: search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card:find_all:page:{page}:size:{page_size}:search:{:?}",
            search
        );

        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found all cards in cache for key: {}", &cache_key);
            self.complete_tracing_success(&tracing_ctx, method, "All cards retrieved from cache")
                .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_all_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully fetched cards")
                    .await;

                let inner = response.into_inner();
                let data: Vec<CardResponse> = inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                info!("fetched {} cards", api_response.data.len());

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch cards")
                    .await;
                error!("find_all failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_active(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;
        let search = &req.search;

        info!(
            "fetching active cards - page: {page}, page_size: {page_size}, search: {:?}",
            search.clone(),
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindActiveCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllCardRequest {
            page,
            page_size,
            search: search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "user:find_by_active:page:{page}:size:{page_size}:search:{}",
            search
        );

        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found active cards in cache for key: {}", &cache_key);
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Active cards retrieved from cache",
            )
            .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_by_active_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched active cards",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                info!("fetched {} active cards", api_response.data.len());

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch active cards")
                    .await;
                error!("find_active failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_trashed(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, HttpError> {
        let page = req.page;
        let page_size = req.page_size;
        let search = &req.search;

        info!(
            "fetching trashed cards - page: {page}, page_size: {page_size}, search: {:?}",
            search
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindTrashedCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllCardRequest {
            page,
            page_size,
            search: search.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "user:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            search
        );

        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found trashed cards in cache for key: {}", &cache_key);
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Trashed cards retrieved from cache",
            )
            .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_by_trashed_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched trashed cards",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                info!("fetched {} trashed cards", api_response.data.len());

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch trashed cards")
                    .await;
                error!("find_trashed failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, HttpError> {
        info!("fetching card by id: {id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindCardById",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("card_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card:find_by_id:id:{id}");
        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card by id in cache for key: {}", cache_key);
            self.complete_tracing_success(&tracing_ctx, method, "Card by id retrieved from cache")
                .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_by_id_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched card by id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - missing data in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let card_response: CardResponse = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    message: inner.message,
                    status: inner.status,
                };

                info!("found card {id}");

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch card by id")
                    .await;
                error!("card {id} - gRPC failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_user_id(&self, user_id: i32) -> Result<ApiResponse<CardResponse>, HttpError> {
        info!("fetching card by user_id: {user_id}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindCardByUserId",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_by_user_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByUserIdCardRequest { user_id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card:find_by_user_id:user_id:{user_id}");
        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card by user_id in cache for key: {cache_key}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Card by user_id retrieved from cache",
            )
            .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_by_user_id_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched card by user id",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("user {user_id} - missing card data in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let card_response: CardResponse = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    message: inner.message,
                    status: inner.status,
                };

                info!("found card for user {user_id}");

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch card by user id",
                )
                .await;
                error!("user {user_id} - gRPC failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_card_number(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<CardResponse>, HttpError> {
        info!("fetching card by card_number: {card_number}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "FindCardByNumber",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "find_by_card_number"),
                KeyValue::new("card_number", card_number.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindByCardNumberRequest {
            card_number: card_number.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card:find_by_card_number:number:{card_number}");
        if let Some(cached_result) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card by number in cache for key: {}", cache_key);
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Card by number retrieved from cache",
            )
            .await;
            return Ok(cached_result);
        }

        match self.client.clone().find_by_card_number(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched card by number",
                )
                .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {card_number} - missing data in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let card_response: CardResponse = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    message: inner.message,
                    status: inner.status,
                };

                info!("found card for number: {card_number}");

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch card by number")
                    .await;
                error!("card {card_number} - gRPC failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardCommandGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, HttpError> {
        info!("creating card for user_id: {}", req.user_id);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "create"),
                KeyValue::new("user_id", req.user_id.to_string()),
            ],
        );

        let date = naive_date_to_timestamp(req.expire_date);

        let mut grpc_req = Request::new(CreateCardRequest {
            user_id: req.user_id,
            card_type: req.card_type.clone(),
            expire_date: Some(date),
            cvv: req.cvv.clone(),
            card_provider: req.card_provider.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully created card")
                    .await;

                let inner = response.into_inner();

                let data = inner.data.ok_or_else(|| {
                    error!("user {} - card data missing in gRPC response", req.user_id);
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                info!("card created successfully for user {}", req.user_id);

                let cache_key = format!("card:find_by_id:{}", data.id);

                let card_response: CardResponse = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    status: inner.status,
                    message: inner.message,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to create card")
                    .await;
                error!("create card failed for user {}: {status:?}", req.user_id);
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, HttpError> {
        let card_id = req
            .card_id
            .ok_or_else(|| HttpError::Internal("card_id is required".to_string()))?;

        info!("updating card id: {card_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "update"),
                KeyValue::new("card_id", card_id.to_string()),
            ],
        );

        let date = naive_date_to_timestamp(req.expire_date);

        let mut grpc_req = Request::new(UpdateCardRequest {
            card_id,
            user_id: req.user_id,
            card_type: req.card_type.clone(),
            expire_date: Some(date),
            cvv: req.cvv.clone(),
            card_provider: req.card_provider.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully updated card")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {card_id} - data missing in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let card_response: CardResponse = data.into();

                let api_response = ApiResponse {
                    data: card_response.clone(),
                    status: inner.status,
                    message: inner.message,
                };

                let cache_key_delete = vec![
                    format!("card:find_by_id:{:?}", req.card_id.clone()),
                    "card:find_all:*".to_string(),
                    "card:find_by_active:*".to_string(),
                    "card:find_by_trashed:*".to_string(),
                ];

                for key_delete in cache_key_delete {
                    self.cache_store.delete_from_cache(&key_delete).await;
                }

                let cache_key = format!("card:find_by_id:{:?}", req.card_id.clone());

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("card {card_id} updated successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to update card")
                    .await;
                error!("update card {card_id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, HttpError> {
        info!("trashing card id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("card_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully trashed card")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - data missing in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let card_response: CardResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_key_delete = vec![
                    format!("card:find_by_id:{id}"),
                    "card:find_all:*".to_string(),
                    "card:find_by_active:*".to_string(),
                    "card:find_by_trashed:*".to_string(),
                ];

                for key_delete in cache_key_delete {
                    self.cache_store.delete_from_cache(&key_delete).await;
                }

                info!("card {id} trashed successfully");
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to trash card")
                    .await;
                error!("trash card {id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, HttpError> {
        info!("restoring card id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("card_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully restored card")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - data missing in gRPC response");
                    HttpError::Internal("Card data is missing in gRPC response".into())
                })?;

                let cache_keys = vec![
                    format!("card:find_by_id:id:{id}"),
                    format!("card:find_by_user_id:user_id:{}", data.user_id),
                    format!("card:find_by_card:number:{}", data.card_number),
                    "card:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let card_response: CardResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: card_response,
                    status: inner.status,
                    message: inner.message,
                };

                info!("card {id} restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to restore card")
                    .await;
                error!("restore card {id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting card id: {id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("card_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().delete_card_permanent(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted card permanently",
                )
                .await;

                let inner = response.into_inner();

                let cache_keys = vec![
                    format!("card:find_by_id:id:{id}"),
                    "card:find_by_card_number:number:*".to_string(),
                    "card:find_all:*".to_string(),
                    "user:find_by_active:*".to_string(),
                    "user:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                info!("card {id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete card permanently",
                )
                .await;
                error!("delete card {id} permanently failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed cards");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllCards",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_card(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully restored all trashed cards",
                )
                .await;

                let inner = response.into_inner();
                info!("all trashed cards restored successfully");

                let cache_keys = vec![
                    "user:find_by_trashed:*",
                    "user:find_by_active:*",
                    "card:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to restore all trashed cards",
                )
                .await;
                error!("restore all cards failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all cards");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllCards",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_card_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully deleted all cards permanently",
                )
                .await;

                let inner = response.into_inner();

                let cache_keys = vec![
                    "user:find_by_trashed:*",
                    "user:find_by_active:*",
                    "card:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                info!("all cards permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to delete all cards permanently",
                )
                .await;
                error!("delete all cards permanently failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsBalanceGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, HttpError> {
        info!("fetching monthly balance for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyBalance",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearBalance { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_balance:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthBalance>>>(&cache_key)
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

        match self.client.clone().find_monthly_balance(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;
                info!(
                    "fetched {} monthly balances for year {year}",
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
                error!("fetch monthly balance for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, HttpError> {
        info!("fetching yearly balance for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyBalance",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearBalance { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_balance:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearlyBalance>>>(&cache_key)
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

        match self.client.clone().find_yearly_balance(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly balance",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearlyBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly balances for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch yearly balance")
                    .await;
                error!("fetch yearly balance for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTopupGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_topup_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!("fetching monthly topup amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTopupAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_topup_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_topup:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
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
            .find_monthly_topup_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;
                info!(
                    "fetched {} monthly topup amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup amount",
                )
                .await;
                error!("fetch monthly topup amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_topup_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!("fetching yearly topup amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTopupAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_topup_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_topup:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
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

        match self.client.clone().find_yearly_topup_amount(grpc_req).await {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;
                info!(
                    "fetched {} yearly topup amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup amount",
                )
                .await;
                error!("fetch yearly topup amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTransactionGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_transaction_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!("fetching monthly TRANSACTION amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTransactionAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_transaction_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transaction:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
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

        match self
            .client
            .clone()
            .find_monthly_transaction_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;
                info!(
                    "fetched {} monthly transaction amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction amount",
                )
                .await;
                error!("fetch monthly TRANSACTION amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_transaction_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!("fetching yearly TRANSACTION amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTransactionAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_transaction_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transaction:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
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

        match self
            .client
            .clone()
            .find_yearly_transaction_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;
                info!(
                    "fetched {} yearly transaction amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction amount",
                )
                .await;
                error!("fetch yearly TRANSACTION amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTransferGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!("fetching monthly TRANSFER amount (sender) for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTransferSenderAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_transfer_sender_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transfer:monthly_sender:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts (sent) in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (sent) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_sender_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transfer sender amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly transfer amounts (sender) for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transfer sender amount",
                )
                .await;
                error!("fetch monthly TRANSFER amount (sender) for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!("fetching yearly TRANSFER amount (sender) for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTransferSenderAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_transfer_sender_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transfer:yearly_sender:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts (sent) in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (sent) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_sender_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transfer sender amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };
                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;
                info!(
                    "fetched {} yearly transfer amounts (sender) for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transfer sender amount",
                )
                .await;
                error!("fetch yearly TRANSFER amount (sender) for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!("fetching monthly TRANSFER amount (receiver) for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTransferReceiverAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_transfer_receiver_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transfer:monthly_receiver:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts (received) in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (received) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_receiver_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transfer receiver amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly transfer amounts (receiver) for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transfer receiver amount",
                )
                .await;
                error!(
                    "fetch monthly TRANSFER amount (receiver) for year {year} failed: {status:?}"
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!("fetching yearly TRANSFER amount (receiver) for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTransferReceiverAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_transfer_receiver_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_transfer:yearly_receiver:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts (received) in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (received) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_receiver_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transfer receiver amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;
                info!(
                    "fetched {} yearly transfer amounts (receiver) for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transfer receiver amount",
                )
                .await;
                error!(
                    "fetch yearly TRANSFER amount (receiver) for year {year} failed: {status:?}"
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsWithdrawGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_withdraw_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!("fetching monthly WITHDRAW amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyWithdrawAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_withdraw_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_withdraw:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
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

        match self
            .client
            .clone()
            .find_monthly_withdraw_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly withdraw amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;
                info!(
                    "fetched {} monthly withdraw amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly withdraw amount",
                )
                .await;
                error!("fetch monthly WITHDRAW amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_withdraw_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!("fetching yearly WITHDRAW amount for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyWithdrawAmount",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_withdraw_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmount { year });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("card_stats_withdraw:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
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

        match self
            .client
            .clone()
            .find_yearly_withdraw_amount(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly withdraw amount",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;
                info!(
                    "fetched {} yearly withdraw amounts for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly withdraw amount",
                )
                .await;
                error!("fetch yearly WITHDRAW amount for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsBalanceByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_balance_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, HttpError> {
        info!(
            "fetching monthly BALANCE for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyBalanceByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_balance_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearBalanceCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_balance:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
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
            .find_monthly_balance_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly balance by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };
                info!(
                    "fetched {} monthly balances for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly balance by card",
                )
                .await;
                error!(
                    "fetch monthly BALANCE for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_balance_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, HttpError> {
        info!(
            "fetching yearly BALANCE for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyBalanceByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_balance_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearBalanceCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_balance:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearlyBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
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
            .find_yearly_balance_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly balance by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearlyBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly balances for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly balance by card",
                )
                .await;
                error!(
                    "fetch yearly BALANCE for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTopupByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_topup_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!(
            "fetching monthly TOPUP amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTopupAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_topup_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_topup:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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
            .find_monthly_topup_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly topup amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly topup amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly topup amount by card",
                )
                .await;
                error!(
                    "fetch monthly TOPUP for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_topup_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!(
            "fetching yearly TOPUP amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTopupAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_topup_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_topup:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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
            .find_yearly_topup_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly topup amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly topup amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly topup amount by card",
                )
                .await;
                error!(
                    "fetch yearly TOPUP for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTransactionByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_transaction_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!(
            "fetching monthly TRANSACTION amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyTransactionAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_transaction_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transaction:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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
            .find_monthly_transaction_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transaction amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly transaction amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transaction amount by card",
                )
                .await;
                error!(
                    "fetch monthly TRANSACTION for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_transaction_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!(
            "fetching yearly TRANSACTION amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyTransactionAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_transaction_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transaction:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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
            .find_yearly_transaction_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transaction amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly transaction amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transaction amount by card",
                )
                .await;
                error!(
                    "fetch yearly TRANSACTION for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsTransferByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_sender_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!(
            "fetching monthly TRANSFER amount (sender) for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountSenderByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_amount_sender_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transfer:monthly_sender:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (sent) in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (sent) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_sender_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transfer sender amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly transfer amounts (sender) for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transfer sender amount by card",
                )
                .await;
                error!(
                    "fetch monthly TRANSFER (sender) for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_sender_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!(
            "fetching yearly TRANSFER amount (sender) for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountSenderByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_amount_sender_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transfer:yearly_sender:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (sent) in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (sent) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_sender_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transfer sender amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly transfer amounts (sender) for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transfer sender amount by card",
                )
                .await;
                error!(
                    "fetch yearly TRANSFER (sender) for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_receiver_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!(
            "fetching monthly TRANSFER amount (receiver) for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyAmountReceiverByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_amount_receiver_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transfer:monthly_receiver:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (received) in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (received) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_transfer_receiver_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly transfer receiver amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly transfer amounts (receiver) for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly transfer receiver amount by card",
                )
                .await;
                error!(
                    "fetch monthly TRANSFER (receiver) for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_receiver_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!(
            "fetching yearly TRANSFER amount (receiver) for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyAmountReceiverByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_amount_receiver_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_transfer:yearly_receiver:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (received) in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (received) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_transfer_receiver_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly transfer receiver amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly transfer amounts (receiver) for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly transfer receiver amount by card",
                )
                .await;
                error!(
                    "fetch yearly TRANSFER (receiver) for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl CardStatsWithdrawByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_withdraw_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, HttpError> {
        info!(
            "fetching monthly WITHDRAW amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetMonthlyWithdrawAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_monthly_withdraw_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_withdraw:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly withdraw amounts in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly withdraw amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_withdraw_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched monthly withdraw amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly withdraw amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch monthly withdraw amount by card",
                )
                .await;
                error!(
                    "fetch monthly WITHDRAW for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_withdraw_amount_bycard(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, HttpError> {
        info!(
            "fetching yearly WITHDRAW amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "GetYearlyWithdrawAmountByCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "get_yearly_withdraw_amount_bycard"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "card_stats_withdraw:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly withdraw amounts in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly withdraw amounts retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_withdraw_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched yearly withdraw amount by card",
                )
                .await;

                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly withdraw amounts for card {} year {}",
                    api_response.data.len(),
                    req.card_number,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.complete_tracing_error(
                    &tracing_ctx,
                    method,
                    "Failed to fetch yearly withdraw amount by card",
                )
                .await;
                error!(
                    "fetch yearly WITHDRAW for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}
