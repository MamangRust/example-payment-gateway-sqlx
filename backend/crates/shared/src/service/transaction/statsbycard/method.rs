use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::method::DynTransactionStatsMethodByCardRepository,
        service::statsbycard::method::TransactionStatsMethodByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse},
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

pub struct TransactionStatsMethodByCardService {
    pub method: DynTransactionStatsMethodByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsMethodByCardService {
    pub fn new(
        method: DynTransactionStatsMethodByCardRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            method,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transaction-stats-method-bycard-service")
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
impl TransactionStatsMethodByCardServiceTrait for TransactionStatsMethodByCardService {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "💳📊 Fetching monthly transaction methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_transaction_methods",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let methods = match self.method.get_monthly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction method records for card {}-{}",
                    methods.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction methods retrieved successfully",
                )
                .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transaction methods for card {}-{}: {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve monthly transaction methods: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionMonthMethodResponse> = methods
            .into_iter()
            .map(TransactionMonthMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction method records for card {}-{}",
            response.data.len(),
            masked_card,
            req.year
        );

        Ok(response)
    }

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📈💳 Fetching yearly transaction methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_transaction_methods",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let methods = match self.method.get_yearly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction method records for card {} ({})",
                    methods.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction methods retrieved successfully",
                )
                .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transaction methods for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly transaction methods: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionYearMethodResponse> = methods
            .into_iter()
            .map(TransactionYearMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction method records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
