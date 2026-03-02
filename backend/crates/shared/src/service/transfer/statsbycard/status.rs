use crate::{
    abstract_trait::transfer::{
        repository::statsbycard::status::DynTransferStatsStatusByCardRepository,
        service::statsbycard::status::TransferStatsStatusByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transfer::{MonthStatusTransferCardNumber, YearStatusTransferCardNumber},
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

pub struct TransferStatsStatusByCardService {
    pub status: DynTransferStatsStatusByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsStatusByCardService {
    pub fn new(
        status: DynTransferStatsStatusByCardRepository,
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
impl TransferStatsStatusByCardServiceTrait for TransferStatsStatusByCardService {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful monthly transfers for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_status_success_by_card",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_success_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_success:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful monthly transfers in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful monthly transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful monthly transfer records for card {} ({}-{})",
                    results.len(),
                    req.card_number,
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successful monthly transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful monthly transfers for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve successful monthly transfers: {:?}", e),
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
                "Successfully retrieved successful transfers for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful monthly transfer records for card {} ({}-{})",
            response.data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful transfers for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_success_by_card",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_success_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_status_success:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful transfers in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful transfer records for card {}",
                    results.len(),
                    req.card_number
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
                error!(
                    "❌ Failed to retrieve yearly successful transfers for card {} in {}: {}",
                    req.card_number, req.year, e
                );
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
            message: format!(
                "Successfully retrieved successful yearly transfers for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful transfer records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed monthly transfers for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_status_failed_by_card",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "month_status_failed_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
                KeyValue::new("month", req.month.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:month_status_failed:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed monthly transfers in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed monthly transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed monthly transfer records for card {} ({}-{})",
                    results.len(),
                    req.card_number,
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Failed monthly transfers retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed monthly transfers for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve failed monthly transfers: {:?}", e),
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
                "Successfully retrieved failed transfers for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed monthly transfer records for card {} ({}-{})",
            response.data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed transfers for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_status_failed_by_card",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_status_failed_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_status_failed:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed transfers in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed transfers retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed transfer records for card {}",
                    results.len(),
                    req.card_number
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
                error!(
                    "❌ Failed to retrieve yearly failed transfers for card {} in {}: {}",
                    req.card_number, req.year, e
                );
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
            message: format!(
                "Successfully retrieved failed yearly transfers for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed transfer records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
