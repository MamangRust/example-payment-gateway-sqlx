use crate::{
    abstract_trait::merchant::{
        repository::stats::amount::DynMerchantStatsAmountRepository,
        service::stats::amount::MerchantStatsAmountServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
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

pub struct MerchantStatsAmountService {
    pub amount: DynMerchantStatsAmountRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsAmountService {
    pub fn new(amount: DynMerchantStatsAmountRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl MerchantStatsAmountServiceTrait for MerchantStatsAmountService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError> {
        info!(
            "📅📊 Fetching monthly transaction amounts for merchant (Year: {})",
            year
        );

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_merchant_amount",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly merchant amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly merchant amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly merchant records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly merchant amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly merchant amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly merchant amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} monthly merchant records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError> {
        info!("📆📈 Fetching yearly transaction amounts for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_merchant_amount",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly merchant amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly merchant amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly merchant records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly merchant amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly merchant amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly merchant amounts for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully retrieved {} yearly merchant records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
