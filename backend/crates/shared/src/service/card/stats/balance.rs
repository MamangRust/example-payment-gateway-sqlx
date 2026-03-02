use crate::{
    abstract_trait::card::{
        repository::stats::balance::DynCardStatsBalanceRepository,
        service::stats::balance::CardStatsBalanceServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
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

pub struct CardStatsBalanceService {
    pub balance: DynCardStatsBalanceRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsBalanceService {
    pub fn new(balance: DynCardStatsBalanceRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            balance,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsBalanceServiceTrait for CardStatsBalanceService {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError> {
        info!("📅 Fetching monthly balance statistics for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_balance:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthBalance>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly balance in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly balance retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_monthly_balance(year).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} monthly balance records for year {year}",
                    balances.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly balance retrieved successfully",
                    )
                    .await;
                balances
            }
            Err(e) => {
                error!(
                    "🗄️ Failed to retrieve monthly balance from repository for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
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
            message: format!("Monthly balance for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} monthly balance records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError> {
        info!("📆 Fetching yearly balance statistics for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_balance:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearlyBalance>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly balance in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly balance retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_yearly_balance(year).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} yearly balance records for year {year}",
                    balances.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly balance retrieved successfully",
                    )
                    .await;
                balances
            }
            Err(e) => {
                error!(
                    "🗄️ Failed to retrieve yearly balance from repository for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
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
            message: format!("Yearly balance for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} yearly balance records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
