use crate::{
    abstract_trait::transfer::{
        repository::stats::status::DynTransferStatsStatusRepository,
        service::stats::status::TransferStatsStatusServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transfer::MonthStatusTransfer,
        responses::{
            ApiResponse, TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
            TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
        },
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

pub struct TransferStatsStatusService {
    pub status: DynTransferStatsStatusRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsStatusService {
    pub fn new(
        status: DynTransferStatsStatusRepository,
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
        global::tracer("transfer-stats-status-service")
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
impl TransferStatsStatusServiceTrait for TransferStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful transfers for month: {}-{}",
            req.year, req.month
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
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_success"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful transfer records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful transfers retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful transfers for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve successful transfers: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransferResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful transfer records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError> {
        info!("📅✅ Fetching yearly successful transfers for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful transfers in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful transfer records for year {year}",
                    results.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful transfers retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly successful transfers for year {year}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly successful transfers: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransferResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful yearly transfers for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful transfer records for {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed transfers for month: {}-{}",
            req.year, req.month
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
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_failed"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed transfer records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed transfers retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed transfers for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve failed transfers: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransferResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed transfer records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError> {
        info!("📅❌ Fetching yearly failed transfers for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed transfers in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transfers retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed transfer records for year {year}",
                    results.len()
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed transfers retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly failed transfers for year {year}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly failed transfers: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseYearStatusFailed> = results
            .into_iter()
            .map(TransferResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed yearly transfers for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed transfer records for {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
