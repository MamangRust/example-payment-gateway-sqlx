use crate::{
    abstract_trait::topup::{
        repository::stats::status::DynTopupStatsStatusRepository,
        service::stats::status::TopupStatsStatusServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::topup::MonthTopupStatus,
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
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

pub struct TopupStatsStatusService {
    pub status: DynTopupStatsStatusRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupStatsStatusService {
    pub fn new(status: DynTopupStatsStatusRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            status,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TopupStatsStatusServiceTrait for TopupStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊 Fetching successful top-ups for month: {} and year: {}",
            req.month, req.year
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
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_status_success"),
                KeyValue::new("month", req.month.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:monthly_status_success:month:{}:year:{}",
            req.month, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful top-ups in cache for month: {}, year: {}",
                req.month, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful top-ups retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} successful top-up records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successful top-ups retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch successful top-ups for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch successful top-ups: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TopupResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful top-ups for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully fetched {} successful top-up records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError> {
        info!("📊 Fetching yearly successful top-ups for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_success",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_status_success"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:yearly_status_success:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly successful top-ups in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful top-ups retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} yearly successful top-up records for {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly successful top-ups retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly successful top-ups for {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch yearly successful top-ups: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseYearStatusSuccess> = results
            .into_iter()
            .map(TopupResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful top-ups for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully fetched {} yearly successful top-up records for {year}",
            response.data.len()
        );

        Ok(response)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊 Fetching failed top-ups for month: {} and year: {}",
            req.month, req.year
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
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_status_failed"),
                KeyValue::new("month", req.month.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:monthly_status_failed:month:{}:year:{}",
            req.month, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed top-ups in cache for month: {}, year: {}",
                req.month, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed top-ups retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} failed top-up records for {}-{}",
                    results.len(),
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Failed top-ups retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch failed top-ups for {}-{}: {e:?}",
                    req.year, req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch failed top-ups: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseMonthStatusFailed> = results
            .into_iter()
            .map(TopupResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed top-ups for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully fetched {} failed top-up records for {}-{}",
            response.data.len(),
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError> {
        info!("📊 Fetching yearly failed top-ups for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_failed",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_status_failed"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut request = Request::new(year);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("topup:yearly_status_failed:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly failed top-ups in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed top-ups retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(year).await {
            Ok(results) => {
                info!(
                    "✅ Successfully fetched {} yearly failed top-up records for {year}",
                    results.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly failed top-ups retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly failed top-ups for {year}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch yearly failed top-ups: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupResponseYearStatusFailed> = results
            .into_iter()
            .map(TopupResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed top-ups for year {year}"),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Successfully fetched {} yearly failed top-up records for {year}",
            response.data.len()
        );

        Ok(response)
    }
}
