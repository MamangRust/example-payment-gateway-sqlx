use crate::{
    abstract_trait::merchant::{
        repository::stats::method::DynMerchantStatsMethodRepository,
        service::stats::method::MerchantStatsMethodServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
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

pub struct MerchantStatsMethodService {
    pub method: DynMerchantStatsMethodRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsMethodService {
    pub fn new(method: DynMerchantStatsMethodRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            method,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl MerchantStatsMethodServiceTrait for MerchantStatsMethodService {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError> {
        info!("📅💳 Fetching monthly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_payment_method",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:monthly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly payment method statistics in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly payment method records for year {year}",
                    methods.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly payment method statistics retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly payment method data for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly payment method statistics: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(30))
            .await;

        info!(
            "✅ Retrieved {} monthly payment method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!("📆💳 Fetching yearly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_payment_method",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("merchant:yearly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly payment method statistics in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly payment method records for year {year}",
                    methods.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly payment method statistics retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly payment method data for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly payment method statistics: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly payment method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
