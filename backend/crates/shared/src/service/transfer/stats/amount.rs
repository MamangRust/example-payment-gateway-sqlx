use crate::{
    abstract_trait::transfer::{
        repository::stats::amount::DynTransferStatsAmountRepository,
        service::stats::amount::TransferStatsAmountServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
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

pub struct TransferStatsAmountService {
    pub amount: DynTransferStatsAmountRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsAmountService {
    pub fn new(amount: DynTransferStatsAmountRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransferStatsAmountServiceTrait for TransferStatsAmountService {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly transfer amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_transfer_amounts",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transfer amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly transfer amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly transfer amounts: {:?}", e),
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
            message: format!("Monthly transfer amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly transfer amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_transfer_amounts",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transfer amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly transfer amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly transfer amounts: {:?}", e),
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
            message: format!("Yearly transfer amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
