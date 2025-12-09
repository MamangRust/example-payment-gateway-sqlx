use crate::{
    abstract_trait::transfer::{
        repository::statsbycard::amount::DynTransferStatsAmountByCardRepository,
        service::statsbycard::amount::TransferStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
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

pub struct TransferStatsAmountByCardService {
    pub amount: DynTransferStatsAmountByCardRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsAmountByCardService {
    pub fn new(
        amount: DynTransferStatsAmountByCardRepository,
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
        global::tracer("transfer-stats-amount-service")
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
impl TransferStatsAmountByCardServiceTrait for TransferStatsAmountByCardService {
    async fn get_monthly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!(
            "💳➡️📊 Fetching monthly transfer amounts (as sender) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_amounts_by_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_amounts_by_sender"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:monthly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (as sender) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts_by_sender_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer records (as sender) for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (as sender) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer amounts (as sender) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly transfer amounts (as sender): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transfer records (as sender) for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
    }

    async fn get_monthly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!(
            "⬅️💳📊 Fetching monthly transfer amounts (as receiver) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_monthly_amounts_by_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_amounts_by_receiver"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:monthly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Monthly transfer amounts (as receiver) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts_by_receiver_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer records (as receiver) for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (as receiver) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer amounts (as receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly transfer amounts (as receiver): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transfer records (as receiver) for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
    }

    async fn get_yearly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!(
            "💳➡️📅 Fetching yearly transfer amounts (as sender) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_amounts_by_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_amounts_by_sender"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (as sender) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts_by_sender_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer records (as sender) for card {}",
                    amounts.len(),
                    req.card_number
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (as sender) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer amounts (as sender) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve yearly transfer amounts (as sender): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transfer records (as sender) for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }

    async fn get_yearly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!(
            "⬅️💳📅 Fetching yearly transfer amounts (as receiver) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.start_tracing(
            "get_yearly_amounts_by_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_amounts_by_receiver"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.complete_tracing_success(
                &tracing_ctx,
                method,
                "Yearly transfer amounts (as receiver) retrieved from cache",
            )
            .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts_by_receiver_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer records (as receiver) for card {}",
                    amounts.len(),
                    req.card_number
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (as receiver) retrieved successfully",
                )
                .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer amounts (as receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve yearly transfer amounts (as receiver): {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transfer records (as receiver) for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
