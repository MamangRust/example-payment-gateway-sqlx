use crate::{
    abstract_trait::withdraw::{
        repository::stats::amount::DynWithdrawStatsAmountRepository,
        service::stats::amount::WithdrawStatsAmountServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
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

pub struct WithdrawStatsAmountService {
    pub amount: DynWithdrawStatsAmountRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawStatsAmountService {
    pub fn new(amount: DynWithdrawStatsAmountRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl WithdrawStatsAmountServiceTrait for WithdrawStatsAmountService {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("withdrawal:monthly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly withdrawal amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly withdrawal amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_withdraws(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly withdrawal records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly withdrawal amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly withdrawal amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly withdrawal amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawMonthlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawMonthlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly withdrawal records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_withdrawals",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("withdrawal:yearly_amounts:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly withdrawal amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly withdrawal amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_withdraws(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly withdrawal records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly withdrawal amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly withdrawal amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly withdrawal amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawYearlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly withdrawal records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
