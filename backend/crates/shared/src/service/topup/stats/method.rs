use crate::{
    abstract_trait::topup::{
        repository::stats::method::DynTopupStatsMethodRepository,
        service::stats::method::TopupStatsMethodServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
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

pub struct TopupStatsMethodService {
    pub method: DynTopupStatsMethodRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupStatsMethodService {
    pub fn new(method: DynTopupStatsMethodRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            method,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TopupStatsMethodServiceTrait for TopupStatsMethodService {
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError> {
        info!("📅💳 Fetching monthly top-up methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_topup_methods",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_methods"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:monthly_methods:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly top-up methods in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly top-up methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_methods(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly top-up method records for year {year}",
                    methods.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly top-up methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve monthly top-up methods for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly top-up methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupMonthMethodResponse> = methods
            .into_iter()
            .map(TopupMonthMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly top-up methods for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly top-up method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError> {
        info!("📆💳 Fetching yearly top-up methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_topup_methods",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_methods"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:yearly_methods:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly top-up methods in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly top-up methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_methods(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly top-up method records for year {year}",
                    methods.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly top-up methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly top-up methods for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly top-up methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupYearlyMethodResponse> = methods
            .into_iter()
            .map(TopupYearlyMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly top-up methods for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly top-up method records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
