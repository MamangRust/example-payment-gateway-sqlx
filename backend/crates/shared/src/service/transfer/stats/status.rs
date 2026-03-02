use crate::{
    abstract_trait::transfer::{
        repository::stats::status::DynTransferStatsStatusRepository,
        service::stats::status::TransferStatsStatusServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transfer::MonthStatusTransfer,
        responses::{
            ApiResponse, TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
            TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct TransferStatsStatusService {
    pub status: DynTransferStatsStatusRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsStatusService {
    pub fn new(status: DynTransferStatsStatusRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            status,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransferStatsStatusServiceTrait for TransferStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful transfers for month: {}-{}",
            req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_status_success",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_success"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful transfer records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successful transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful transfers for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve successful transfers: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransferResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful transfer records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError> {
        info!("📅✅ Fetching yearly successful transfers for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful transfers in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful transfer records for year {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly successful transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly successful transfers for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly successful transfers: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransferResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful yearly transfers for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful transfer records for {year}",
            response.data.len(),
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed transfers for month: {}-{}",
            req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_status_failed",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_failed"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transfers in cache for month: {}-{}",
                req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed transfer records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Failed transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed transfers for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve failed transfers: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransferResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed transfer records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError> {
        info!("📅❌ Fetching yearly failed transfers for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transfer:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed transfers in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed transfer records for year {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly failed transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly failed transfers for year {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly failed transfers: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferResponseYearStatusFailed> = results
            .into_iter()
            .map(TransferResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed yearly transfers for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed transfer records for {year}",
            response.data.len(),
        );

        Ok(response)
    }
}
