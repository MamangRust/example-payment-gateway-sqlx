use crate::{
    abstract_trait::card::{
        repository::statsbycard::transfer::DynCardStatsTransferByCardRepository,
        service::statsbycard::transfer::CardStatsTransferByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
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

pub struct CardStatsTransferByCardService {
    pub transfer: DynCardStatsTransferByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsTransferByCardService {
    pub fn new(
        transfer: DynCardStatsTransferByCardRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            transfer,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("card-stats-bycard-transfer-service")
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
impl CardStatsTransferByCardServiceTrait for CardStatsTransferByCardService {
    async fn get_monthly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📤 Fetching monthly transfer amounts (sent) for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_amount_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_sender"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.transfer.get_monthly_amount_sender(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer (sender) records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (sent) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly 'sent' transfer data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly transfer amounts (sent): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (sent) for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly 'sent' transfer records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_yearly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📈📤 Fetching yearly transfer amounts (sent) for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_amount_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_sender"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.transfer.get_yearly_amount_sender(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer (sender) records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (sent) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly 'sent' transfer data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly transfer amounts (sent): {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (sent) for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly 'sent' transfer records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_monthly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📥 Fetching monthly transfer amounts (received) for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_amount_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_receiver"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.transfer.get_monthly_amount_receiver(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer (receiver) records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (received) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly 'received' transfer data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly transfer amounts (received): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (received) for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly 'received' transfer records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_yearly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📈📥 Fetching yearly transfer amounts (received) for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_amount_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_receiver"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.transfer.get_yearly_amount_receiver(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer (receiver) records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (received) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly 'received' transfer data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve yearly transfer amounts (received): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (received) for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly 'received' transfer records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }
}
