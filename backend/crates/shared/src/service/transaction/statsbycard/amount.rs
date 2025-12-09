use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::amount::DynTransactionStatsAmountByCardRepository,
        service::statsbycard::amount::TransactionStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse},
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

pub struct TransactionStatsAmountByCardService {
    pub amount: DynTransactionStatsAmountByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsAmountByCardService {
    pub fn new(
        amount: DynTransactionStatsAmountByCardRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            amount,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transaction-stats-amount-bycard-service")
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
impl TransactionStatsAmountByCardServiceTrait for TransactionStatsAmountByCardService {
    async fn get_monthly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "💳📊 Fetching monthly transaction amounts for card: ({}-{})",
            masked_card, req.year,
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_transaction_amounts",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_monthly_amounts(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction amount records for card {}-{}",
                    amounts.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transaction amounts for card {}-{}: {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve monthly transaction amounts: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionMonthAmountResponse> = amounts
            .into_iter()
            .map(TransactionMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for card in {}-{} retrieved successfully",
                masked_card, req.year,
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction amount records for card {}-{}",
            response.data.len(),
            masked_card,
            req.year,
        );

        Ok(response)
    }

    async fn get_yearly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📈💳 Fetching yearly transaction amounts for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_transaction_amounts",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_yearly_amounts(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction amount records for card {} ({})",
                    amounts.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transaction amounts for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly transaction amounts: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionYearlyAmountResponse> = amounts
            .into_iter()
            .map(TransactionYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for card {} in {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction amount records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
