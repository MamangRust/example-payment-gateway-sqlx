use crate::{
    abstract_trait::withdraw::{
        repository::statsbycard::amount::DynWithdrawStatsAmountByCardRepository,
        service::statsbycard::amount::WithdrawStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
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

pub struct WithdrawStatsAmountByCardService {
    pub amount: DynWithdrawStatsAmountByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawStatsAmountByCardService {
    pub fn new(
        amount: DynWithdrawStatsAmountByCardRepository,
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
        global::tracer("withdraw-stats-amount-bycard-service")
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
impl WithdrawStatsAmountByCardServiceTrait for WithdrawStatsAmountByCardService {
    async fn get_monthly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_withdrawal_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "monthly_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_monthly_by_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly withdrawal records for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly withdrawal amounts retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly withdrawal amounts for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve monthly withdrawal amounts: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawMonthlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawMonthlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly withdrawal amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly withdrawal records for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
    }

    async fn get_yearly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_withdrawal_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_yearly_by_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly withdrawal records for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly withdrawal amounts retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly withdrawal amounts for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly withdrawal amounts: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawYearlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly withdrawal amounts for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly withdrawal records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
