use crate::{
    abstract_trait::withdraw::{
        repository::stats::amount::DynWithdrawStatsAmountRepository,
        service::stats::amount::WithdrawStatsAmountServiceTrait,
    },
    cache::CacheStore,
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    errors::ServiceError,
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

pub struct WithdrawStatsAmountService {
    pub amount: DynWithdrawStatsAmountRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawStatsAmountService {
    pub fn new(
        amount: DynWithdrawStatsAmountRepository,
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
        global::tracer("withdraw-stats-amount-service")
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
impl WithdrawStatsAmountServiceTrait for WithdrawStatsAmountService {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_monthly_withdraws(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly withdrawal records for year {year}",
                    amounts.len()
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
                error!("❌ Failed to retrieve monthly withdrawal amounts for year {year}: {e:?}");
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
            message: format!("Monthly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly withdrawal records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

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

        let amounts = match self.amount.get_yearly_withdraws(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly withdrawal records for year {year}",
                    amounts.len()
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
                error!("❌ Failed to retrieve yearly withdrawal amounts for year {year}: {e:?}");
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
            message: format!("Yearly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly withdrawal records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
