use crate::{
    abstract_trait::topup::{
        repository::statsbycard::status::DynTopupStatsStatusByCardRepository,
        service::statsbycard::status::TopupStatsStatusByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::topup::{MonthTopupStatusCardNumber, YearTopupStatusCardNumber},
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
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
use validator::Validate;

pub struct TopupStatsStatusByCardService {
    pub status: DynTopupStatsStatusByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupStatsStatusByCardService {
    pub fn new(
        status: DynTopupStatsStatusByCardRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            status,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("topup-stats-status-service")
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
impl TopupStatsStatusByCardServiceTrait for TopupStatsStatusByCardService {
    async fn get_month_status_success(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊✅ Fetching successful top-ups for card: {} ({}-{})",
            masked_card, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_month_status_success",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_status_success"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} successful top-up records for card {} ({}-{})",
                    results.len(),
                    masked_card,
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful top-ups retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch successful top-ups for card {} ({}-{}): {e:?}",
                    masked_card, req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch successful top-ups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TopupResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful top-ups for card {} in {}-{}",
                masked_card, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful top-up records for card {} ({}-{})",
            response.data.len(),
            masked_card,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📅✅ Fetching yearly successful top-ups for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let results = match self.status.get_yearly_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} yearly successful top-up records for card {} ({})",
                    results.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful top-ups retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly successful top-ups for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch yearly successful top-ups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseYearStatusSuccess> = results
            .into_iter()
            .map(TopupResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved yearly successful top-ups for card {} in {}",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful top-up records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊❌ Fetching failed top-ups for card: {} ({}-{})",
            masked_card, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_month_status_failed",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_status_failed"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} failed top-up records for card {} ({}-{})",
                    results.len(),
                    masked_card,
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed top-ups retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch failed top-ups for card {} ({}-{}): {e:?}",
                    masked_card, req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch failed top-ups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseMonthStatusFailed> = results
            .into_iter()
            .map(TopupResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed top-ups for card {} in {}-{}",
                masked_card, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed top-up records for card {} ({}-{})",
            response.data.len(),
            masked_card,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📅❌ Fetching yearly failed top-ups for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let results = match self.status.get_yearly_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} yearly failed top-up records for card {} ({})",
                    results.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed top-ups retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly failed top-ups for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to fetch yearly failed top-ups: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseYearStatusFailed> = results
            .into_iter()
            .map(TopupResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved yearly failed top-ups for card {} in {}",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed top-up records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
