use crate::{
    abstract_trait::transaction::{
        repository::stats::amount::DynTransactionStatsAmountRepository,
        service::stats::amount::TransactionStatsAmountServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{
        ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse,
    },
    errors::ServiceError,
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};

pub struct TransactionStatsAmountService {
    pub amount: DynTransactionStatsAmountRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsAmountService {
    pub fn new(
        amount: DynTransactionStatsAmountRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransactionStatsAmountServiceTrait for TransactionStatsAmountService {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_transaction_amounts",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transaction amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transaction amounts for year {year}: {:?}",
                    e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly transaction amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionMonthAmountResponse> = amounts
            .into_iter()
            .map(TransactionMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly transaction amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction records for year {year}",
            response.data.len()
        );

        Ok(response)
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError> {
        info!("📈 Fetching yearly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_transaction_amounts",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transaction amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly transaction amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly transaction amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionYearlyAmountResponse> = amounts
            .into_iter()
            .map(TransactionYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly transaction amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction records for year {year}",
            response.data.len()
        );

        Ok(response)
    }
}
