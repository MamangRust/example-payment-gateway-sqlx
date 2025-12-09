use crate::{
    abstract_trait::saldo::{
        repository::stats::total::DynSaldoTotalBalanceRepository,
        service::stats::total::SaldoTotalBalanceServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::saldo::MonthTotalSaldoBalance,
        responses::{ApiResponse, SaldoMonthTotalBalanceResponse, SaldoYearTotalBalanceResponse},
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

pub struct SaldoTotalBalanceService {
    pub total_balance: DynSaldoTotalBalanceRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl SaldoTotalBalanceService {
    pub fn new(
        total_balance: DynSaldoTotalBalanceRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            total_balance,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("saldo-stats-total-balance-service")
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
impl SaldoTotalBalanceServiceTrait for SaldoTotalBalanceService {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, ServiceError> {
        info!("📅💵 Fetching monthly total balance for year: {}", req.year);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_month_total_balance",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "monthly_total_balance"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("saldo:monthly_total_balance:year:{}", req.year);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total balance in cache for year: {}",
                req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly total balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let balances = match self.total_balance.get_month_total_balance(req).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} monthly total balance records for year {}",
                    balances.len(),
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total balance retrieved successfully",
                )
                .await;
                balances
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly total balance for year {}: {e:?}",
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve monthly total balance: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<SaldoMonthTotalBalanceResponse> = balances
            .into_iter()
            .map(SaldoMonthTotalBalanceResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total balance for year {} retrieved successfully",
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly total balance records for year {}",
            response.data.len(),
            req.year
        );

        Ok(response)
    }

    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, ServiceError> {
        info!("📆💵 Fetching yearly total balance for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_year_total_balance",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "yearly_total_balance"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("saldo:yearly_total_balance:year:{}", year);

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly total balance in cache for year: {year}");
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly total balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let balances = match self.total_balance.get_year_total_balance(year).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} yearly total balance records for year {}",
                    balances.len(),
                    year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total balance retrieved successfully",
                )
                .await;
                balances
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly total balance for year {year}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly total balance: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<SaldoYearTotalBalanceResponse> = balances
            .into_iter()
            .map(SaldoYearTotalBalanceResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly total balance for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly total balance records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
