use crate::{
    abstract_trait::card::{
        repository::statsbycard::balance::DynCardStatsBalanceByCardRepository,
        service::statsbycard::balance::CardStatsBalanceByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
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

pub struct CardStatsBalanceByCardService {
    pub balance: DynCardStatsBalanceByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsBalanceByCardService {
    pub fn new(
        balance: DynCardStatsBalanceByCardRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            balance,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("card-stats-bycard-balance-service")
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
impl CardStatsBalanceByCardServiceTrait for CardStatsBalanceByCardService {
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError> {
        info!(
            "💳📅 Fetching monthly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_balance:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_monthly_balance(req).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} monthly balance records for card {}",
                    balances.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly balance retrieved successfully",
                )
                .await;
                balances
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly balance for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve monthly balance: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseMonthBalance> = balances
            .into_iter()
            .map(CardResponseMonthBalance::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly balance for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly balance records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError> {
        info!(
            "💳📆 Fetching yearly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_balance:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearlyBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly balance retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_yearly_balance(req).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} yearly balance records for card {}",
                    balances.len(),
                    mask_card_number(&req.card_number)
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly balance retrieved successfully",
                )
                .await;
                balances
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly balance for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly balance: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseYearlyBalance> = balances
            .into_iter()
            .map(CardResponseYearlyBalance::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly balance for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::hours(1))
            .await;

        info!(
            "✅ Retrieved {} yearly balance records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }
}
