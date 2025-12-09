use crate::{
    abstract_trait::merchant::{
        repository::stats::method::DynMerchantStatsMethodRepository,
        service::stats::method::MerchantStatsMethodServiceTrait,
    },
    cache::CacheStore,
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
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

pub struct MerchantStatsMethodService {
    pub method: DynMerchantStatsMethodRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsMethodService {
    pub fn new(
        method: DynMerchantStatsMethodRepository,
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
        global::tracer("merchant-stats-method-service")
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
impl MerchantStatsMethodServiceTrait for MerchantStatsMethodService {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError> {
        info!("📅💳 Fetching monthly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_payment_method",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:monthly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly payment method statistics in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly payment method statistics retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly payment method records for year {year}",
                    methods.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics retrieved successfully",
                )
                .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly payment method data for year {year}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly payment method statistics: {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(30))
            .await;

        info!(
            "✅ Retrieved {} monthly payment method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!("📆💳 Fetching yearly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_payment_method",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:yearly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly payment method statistics in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly payment method statistics retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly payment method records for year {year}",
                    methods.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics retrieved successfully",
                )
                .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly payment method data for year {year}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve yearly payment method statistics: {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly payment method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
