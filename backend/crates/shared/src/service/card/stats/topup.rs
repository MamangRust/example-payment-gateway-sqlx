use crate::{
    abstract_trait::card::{
        repository::stats::topup::DynCardStatsTopupRepository,
        service::stats::topup::CardStatsTopupServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
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

pub struct CardStatsTopupService {
    pub topup: DynCardStatsTopupRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsTopupService {
    pub fn new(topup: DynCardStatsTopupRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            topup,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsTopupServiceTrait for CardStatsTopupService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("📅 Fetching monthly top-up amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amount",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_topup:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly top-up amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly top-up amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.topup.get_monthly_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly top-up records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly top-up amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("🗄️ Failed to retrieve monthly top-up data for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly top-up amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} monthly top-up records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📆 Fetching yearly top-up amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amount",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_topup:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly top-up amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly top-up amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.topup.get_yearly_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly top-up records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly top-up amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("🗄️ Failed to retrieve yearly top-up data for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly top-up amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} yearly top-up records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
