use crate::{
    abstract_trait::card::{
        repository::query::DynCardQueryRepository, service::query::CardQueryServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::card::FindAllCards,
        responses::{
            ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt, Pagination,
        },
    },
    errors::{RepositoryError, ServiceError},
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

pub struct CardQueryService {
    pub query: DynCardQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardQueryService {
    pub fn new(query: DynCardQueryRepository, cache_store: Arc<CacheStore>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            query,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("card-query-service")
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
impl CardQueryServiceTrait for CardQueryService {
    async fn find_all(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_all_cards",
            vec![
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} cards in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (cards, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} cards", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all cards: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch all cards: {e:?}"),
                )
                .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponse> = cards.into_iter().map(CardResponse::from).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Cards retrieved successfully".to_string(),
            data: card_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} cards (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_active(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Fetching active cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_active_cards",
            vec![
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card:find_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active cards in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (cards, total_items) = match self.query.find_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Retrieved {} active cards", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active cards: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch active cards: {e:?}"),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponseDeleteAt> =
            cards.into_iter().map(|c| c.into()).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active cards retrieved successfully".to_string(),
            data: card_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} active cards (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_trashed(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️  Fetching trashed cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search_str.clone()
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_trashed_cards",
            vec![
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card:find_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<CardResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("🗑️  Found {} trashed cards in cache", cache.data.len());
            info!("{log_msg}");
            self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (cards, total_items) = match self.query.find_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("🗑️  Found {} trashed cards", res.0.len());
                info!("{log_msg}");
                self.complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed cards: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("❌ Failed to fetch trashed cards: {e:?}"),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponseDeleteAt> =
            cards.into_iter().map(|c| c.into()).collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed cards retrieved successfully".to_string(),
            data: card_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "🗑️  Found {} trashed cards (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!("🔍 Finding card by ID: {id}");

        let method = Method::Get;

        let tracing_ctx =
            self.start_tracing("find_by_id", vec![KeyValue::new("id", id.to_string())]);

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card in cache");
            self.complete_tracing_success(&tracing_ctx, method, "Card retrieved from cache")
                .await;
            return Ok(cache);
        }

        let card = match self.query.find_by_id(id).await {
            Ok(card) => {
                info!("✅ Find by id card cache");
                self.complete_tracing_success(&tracing_ctx, method, "Find by id card cache")
                    .await;

                card
            }
            Err(e) => {
                error!("❌ Database error while finding card ID {id}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let card_response = CardResponse::from(card);

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Card retrieved successfully".to_string(),
            data: card_response,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!("✅ Found card: '{}' (ID: {id})", response.data.card_number);

        Ok(response)
    }

    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!("👥 Finding card for user ID: {}", user_id);

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_by_user_id",
            vec![KeyValue::new("user_id", user_id.to_string())],
        );

        let mut request = Request::new(user_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card:find_by_user_id:user_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card for user in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Card for user retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let card = match self.query.find_by_user_id(user_id).await {
            Ok(card) => card,
            Err(e) => {
                error!("❌ Failed to fetch card for user ID {user_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch card for user ID {user_id}"),
                )
                .await;
                return Err(ServiceError::InternalServerError(e.to_string()));
            }
        };

        let response_data = CardResponse::from(card);

        let response = ApiResponse {
            status: "success".into(),
            message: "Card by user ID retrieved successfully".into(),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!("✅ Found card for user ID {user_id}");

        Ok(response)
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!(
            "💳 Finding card by card number: {}",
            mask_card_number(card_number)
        );

        let method = Method::Get;

        let tracing_ctx = self.start_tracing(
            "find_by_card",
            vec![KeyValue::new("card_number", mask_card_number(card_number))],
        );

        let mut request = Request::new(card_number.to_string());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card:find_by_card:number:{}", mask_card_number(card_number));

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<CardResponse>>(&cache_key)
            .await
        {
            info!("✅ Found card by number in cache");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Card by number retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let card = match self.query.find_by_card(card_number).await {
            Ok(card) => card,
            Err(e) => {
                let error_msg = match e {
                    RepositoryError::NotFound => {
                        info!(
                            "ℹ️  Card with number {} not found",
                            mask_card_number(card_number)
                        );
                        "Card not found"
                    }
                    _ => {
                        error!(
                            "❌ Error fetching card by number {}: {e:?}",
                            mask_card_number(card_number),
                        );
                        "Database error"
                    }
                };

                self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                    .await;

                return match e {
                    RepositoryError::NotFound => {
                        Err(ServiceError::NotFound("Card not found".to_string()))
                    }
                    _ => Err(ServiceError::InternalServerError(e.to_string())),
                };
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Card retrieved by card number".to_string(),
            data: CardResponse::from(card),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Card with number {} retrieved successfully",
            mask_card_number(&response.data.card_number)
        );

        Ok(response)
    }
}
