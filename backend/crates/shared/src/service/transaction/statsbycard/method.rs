use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::method::DynTransactionStatsMethodByCardRepository,
        service::statsbycard::method::TransactionStatsMethodByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionStatsMethodByCardService {
    pub method: DynTransactionStatsMethodByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsMethodByCardService {
    pub fn new(
        method: DynTransactionStatsMethodByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            method,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransactionStatsMethodByCardServiceTrait for TransactionStatsMethodByCardService {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "💳📊 Fetching monthly transaction methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_transaction_methods",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:monthly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction methods in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction method records for card {}-{}",
                    methods.len(),
                    masked_card,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transaction methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transaction methods for card {}-{}: {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly transaction methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionMonthMethodResponse> = methods
            .into_iter()
            .map(TransactionMonthMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction method records for card {}-{}",
            response.data.len(),
            masked_card,
            req.year
        );

        Ok(response)
    }

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📈💳 Fetching yearly transaction methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_transaction_methods",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:yearly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction methods in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction method records for card {} ({})",
                    methods.len(),
                    masked_card,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transaction methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transaction methods for card {} ({}): {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly transaction methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionYearMethodResponse> = methods
            .into_iter()
            .map(TransactionYearMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction method records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
