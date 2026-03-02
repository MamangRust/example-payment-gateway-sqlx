use crate::{
    abstract_trait::transaction::{
        repository::stats::method::DynTransactionStatsMethodRepository,
        service::stats::method::TransactionStatsMethodServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{
        ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse,
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

pub struct TransactionStatsMethodService {
    pub method: DynTransactionStatsMethodRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsMethodService {
    pub fn new(
        method: DynTransactionStatsMethodRepository,
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
impl TransactionStatsMethodServiceTrait for TransactionStatsMethodService {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError> {
        info!("📊 Fetching monthly transaction methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_transaction_method",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:monthly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transaction methods in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction method records for year {year}",
                    methods.len()
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
                error!("❌ Failed to retrieve monthly transaction methods for year {year}: {e:?}");
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
            message: format!("Monthly transaction methods for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction method records for year {year}",
            response.data.len()
        );

        Ok(response)
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError> {
        info!("📈📊 Fetching yearly transaction methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_transaction_method",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:yearly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionYearMethodResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transaction methods in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(year).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction method records for year {year}",
                    methods.len()
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
                error!("❌ Failed to retrieve yearly transaction methods for year {year}: {e:?}");
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
            message: format!("Yearly transaction methods for year {year} retrieved successfully"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction method records for year {year}",
            response.data.len()
        );

        Ok(response)
    }
}
