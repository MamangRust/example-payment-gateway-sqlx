use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::status::DynTransactionStatsStatusByCardRepository,
        service::statsbycard::status::TransactionStatsStatusByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transaction::{
            MonthStatusTransactionCardNumber, YearStatusTransactionCardNumber,
        },
        responses::{
            ApiResponse, TransactionResponseMonthStatusFailed,
            TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
            TransactionResponseYearStatusSuccess,
        },
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

pub struct TransactionStatsStatusByCardService {
    pub status: DynTransactionStatsStatusByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsStatusByCardService {
    pub fn new(
        status: DynTransactionStatsStatusByCardRepository,
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
        global::tracer("transaction-stats-status-service")
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
impl TransactionStatsStatusByCardServiceTrait for TransactionStatsStatusByCardService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊✅ Fetching successful transactions for card: {} ({}-{})",
            masked_card, req.year, req.month
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
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_status_success"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:monthly_status_success:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transactions in cache for card: {} ({}-{})",
                masked_card, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful transaction records for card {} ({}-{})",
                    results.len(),
                    masked_card,
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful transactions retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful transactions for card {} ({}-{}): {e:?}",
                    masked_card, req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve successful transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transactions for card {} ({}-{})",
                masked_card, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful transaction records for card {} ({}-{})",
            response.data.len(),
            masked_card,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊✅ Fetching yearly successful transactions for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:yearly_status_success:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful transactions in cache for card: {} ({})",
                masked_card, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly successful transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful transaction records for card {} ({})",
                    results.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful transactions retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly successful transactions for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly successful transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transactions for card {} in {}",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful transaction records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊❌ Fetching failed transactions for card: {} ({}-{})",
            masked_card, req.year, req.month
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
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_status_failed"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:monthly_status_failed:card:{}:year:{}:month:{}",
            masked_card, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transactions in cache for card: {} ({}-{})",
                masked_card, req.year, req.month
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed transaction records for card {} ({}-{})",
                    results.len(),
                    masked_card,
                    req.year,
                    req.month
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed transactions retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed transactions for card {} ({}-{}): {e:?}",
                    masked_card, req.year, req.month
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve failed transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransactionResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transactions for card {} ({}-{})",
                masked_card, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed transaction records for card {} ({}-{})",
            response.data.len(),
            masked_card,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊❌ Fetching yearly failed transactions for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:yearly_status_failed:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed transactions in cache for card: {} ({})",
                masked_card, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly failed transactions retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed transaction records for card {} ({})",
                    results.len(),
                    masked_card,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed transactions retrieved successfully",
                )
                .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly failed transactions for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to retrieve yearly failed transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseYearStatusFailed> = results
            .into_iter()
            .map(TransactionResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transactions for card {} in {}",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed transaction records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
