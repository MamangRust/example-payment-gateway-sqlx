use crate::{
    abstract_trait::card::{
        repository::stats::transfer::DynCardStatsTransferRepository,
        service::stats::transfer::CardStatsTransferServiceTrait,
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

pub struct CardStatsTransferService {
    pub transfer: DynCardStatsTransferRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsTransferService {
    pub fn new(transfer: DynCardStatsTransferRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            transfer,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsTransferServiceTrait for CardStatsTransferService {
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("📤 Fetching monthly transfer amounts (sent) for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amount_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_sender"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_transfer:monthly_sender:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts (sent) in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (sent) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.transfer.get_monthly_amount_sender(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer (sender) records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transfer amounts (sent) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer (sender) data for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transfer amounts (sent): {:?}",
                            e
                        ),
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
            message: format!(
                "Monthly transfer amounts (sent) for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly 'sent' transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📈📤 Fetching yearly transfer amounts (sent) for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amount_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_sender"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_sender:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts (sent) in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (sent) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.transfer.get_yearly_amount_sender(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer (sender) records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transfer amounts (sent) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer (sender) data for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly transfer amounts (sent): {:?}", e),
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
            message: format!(
                "Yearly transfer amounts (sent) for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly 'sent' transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "📥 Fetching monthly transfer amounts (received) for year: {}",
            year
        );

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amount_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_receiver"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_transfer:monthly_receiver:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly transfer amounts (received) in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (received) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.transfer.get_monthly_amount_receiver(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer (receiver) records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transfer amounts (received) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer (receiver) data for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transfer amounts (received): {:?}",
                            e
                        ),
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
            message: format!(
                "Monthly transfer amounts (received) for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly 'received' transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📈📥 Fetching yearly transfer amounts (received) for year: {year}",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amount_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_receiver"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("card_stats_transfer:yearly_receiver:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly transfer amounts (received) in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (received) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.transfer.get_yearly_amount_receiver(year).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer (receiver) records for year {year}",
                    amounts.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transfer amounts (received) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer (receiver) data for year {year}: {e:?}"
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly transfer amounts (received): {:?}",
                            e
                        ),
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
            message: format!(
                "Yearly transfer amounts (received) for year {year} retrieved successfully"
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly 'received' transfer records for year {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
