use crate::{
    abstract_trait::merchant::{
        repository::stats::totalamount::DynMerchantStatsTotalAmountRepository,
        service::stats::totalamount::MerchantStatsTotalAmountServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
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

pub struct MerchantStatsTotalAmountService {
    pub method: DynMerchantStatsTotalAmountRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsTotalAmountService {
    pub fn new(
        method: DynMerchantStatsTotalAmountRepository,
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
impl MerchantStatsTotalAmountServiceTrait for MerchantStatsTotalAmountService {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError> {
        info!("📅💰 Fetching monthly total transaction amounts for merchants (Year: {year})",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_total_amount",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_total"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:monthly_total:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly total transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.method.get_monthly_total_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly total amount records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly total transaction amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly total amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly total transaction amounts: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly total amount records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!("📆💰 Fetching yearly total transaction amounts for merchants (Year: {year})",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_total_amount",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_total"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:yearly_total:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly total transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.method.get_yearly_total_amount(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly total amount records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly total transaction amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly total amounts for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly total transaction amounts: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly total amount records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
