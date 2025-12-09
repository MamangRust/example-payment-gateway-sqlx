use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::totalamount::DynMerchantStatsTotalAmountByApiKeyRepository,
        service::statsbyapikey::totalamount::MerchantStatsTotalAmountByApiKeyServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::merchant::MonthYearTotalAmountApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::{ServiceError, format_validation_errors},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_api_key,
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

pub struct MerchantStatsTotalAmountByApiKeyService {
    pub total_amount: DynMerchantStatsTotalAmountByApiKeyRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsTotalAmountByApiKeyService {
    pub fn new(
        total_amount: DynMerchantStatsTotalAmountByApiKeyRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            total_amount,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("merchant-stats-total-amount-byapikey-service")
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
impl MerchantStatsTotalAmountByApiKeyServiceTrait for MerchantStatsTotalAmountByApiKeyService {
    async fn find_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError> {
        info!(
            "📅💰 Fetching monthly total transaction amounts by API key (Year: {}) | api_key: {}",
            req.year,
            mask_api_key(&req.api_key)
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_monthly_total_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_total_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly total transaction amounts by API key retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_monthly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly total amount records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts by API key retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly total amounts for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly total transaction amounts by API key: {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly total amount records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }

    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!(
            "📆💰 Fetching yearly total transaction amounts by API key (Year: {}) | api_key: {}",
            req.year,
            mask_api_key(&req.api_key)
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "find_yearly_total_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_total_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly total transaction amounts by API key retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_yearly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly total amount records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts by API key retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly total amounts for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve yearly total transaction amounts by API key: {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly total amount records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }
}
