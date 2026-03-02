use crate::{
    abstract_trait::transaction::{
        repository::stats::status::DynTransactionStatsStatusRepository,
        service::stats::status::TransactionStatsStatusServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transaction::MonthStatusTransaction,
        responses::{
            ApiResponse, TransactionResponseMonthStatusFailed,
            TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
            TransactionResponseYearStatusSuccess,
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

pub struct TransactionStatsStatusService {
    pub status: DynTransactionStatsStatusRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionStatsStatusService {
    pub fn new(
        status: DynTransactionStatsStatusRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            status,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransactionStatsStatusServiceTrait for TransactionStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful transactions for month: {}-{}",
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
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_status_success"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:monthly_status_success:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful transactions in cache for month: {}-{}",
                req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful transaction records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successful transactions retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful transactions for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve successful transactions: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transactions for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful transaction records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError> {
        info!("📊✅ Fetching yearly successful transactions for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful transactions in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful transaction records for {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly successful transactions retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly successful transactions for {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly successful transactions: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful transactions for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful transaction records for year {year}",
            response.data.len()
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed transactions for month: {}-{}",
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
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "monthly_status_failed"),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:monthly_status_failed:year:{}:month:{}",
            req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed transactions in cache for month: {}-{}",
                req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed transaction records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Failed transactions retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed transactions for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve failed transactions: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransactionResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transactions for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed transaction records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError> {
        info!("📊❌ Fetching yearly failed transactions for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed transactions in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed transaction records for {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly failed transactions retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to retrieve yearly failed transactions for {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly failed transactions: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransactionResponseYearStatusFailed> = results
            .into_iter()
            .map(TransactionResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed transactions for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed transaction records for year {year}",
            response.data.len()
        );

        Ok(response)
    }
}
