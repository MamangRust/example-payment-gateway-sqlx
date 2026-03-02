use crate::{
    abstract_trait::withdraw::{
        repository::statsbycard::status::DynWithdrawStatsStatusByCardRepository,
        service::statsbycard::status::WithdrawStatsStatusByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
        responses::{
            ApiResponse, WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
            WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
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

pub struct WithdrawStatsStatusByCardService {
    pub status: DynWithdrawStatsStatusByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawStatsStatusByCardService {
    pub fn new(
        status: DynWithdrawStatsStatusByCardRepository,
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
impl WithdrawStatsStatusByCardServiceTrait for WithdrawStatsStatusByCardService {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful monthly withdrawals for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_withdraw_status_success_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
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
            "withdrawal:month_status_success:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found successful monthly withdrawals in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successful monthly withdrawals retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_success_by_card(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} successful monthly withdrawal records for card {} ({}-{})",
                    results.len(),
                    req.card_number,
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successful monthly withdrawals retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve successful monthly withdrawals for card {} ({}-{}): {:?}",
                    req.card_number, req.year, req.month, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve successful monthly withdrawals: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawResponseMonthStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful withdrawals for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} successful monthly withdrawal records for card {} ({}-{})",
            response.data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful withdrawals for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_withdraw_status_success_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_status_success_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "withdrawal:yearly_status_success:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly successful withdrawals in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly successful withdrawals retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_success_by_card(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly successful withdrawal records for card {}",
                    results.len(),
                    req.card_number
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly successful withdrawals retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly successful withdrawals for card {} in {}: {:?}",
                    req.card_number, req.year, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly successful withdrawals: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawResponseYearStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseYearStatusSuccess::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly withdrawals for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly successful withdrawal records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed monthly withdrawals for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_month_withdraw_status_failed_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
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
            "withdrawal:month_status_failed:card:{}:year:{}:month:{}",
            req.card_number, req.year, req.month
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found failed monthly withdrawals in cache for card: {} ({}-{})",
                req.card_number, req.year, req.month
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Failed monthly withdrawals retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_month_status_failed_by_card(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} failed monthly withdrawal records for card {} ({}-{})",
                    results.len(),
                    req.card_number,
                    req.year,
                    req.month
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Failed monthly withdrawals retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve failed monthly withdrawals for card {} ({}-{}): {:?}",
                    req.card_number, req.year, req.month, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve failed monthly withdrawals: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawResponseMonthStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed withdrawals for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} failed monthly withdrawal records for card {} ({}-{})",
            response.data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(response)
    }

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed withdrawals for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_withdraw_status_failed_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_status_failed_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "withdrawal:yearly_status_failed:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly failed withdrawals in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly failed withdrawals retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let results = match self.status.get_yearly_status_failed_by_card(req).await {
            Ok(results) => {
                info!(
                    "✅ Successfully retrieved {} yearly failed withdrawal records for card {}",
                    results.len(),
                    req.card_number
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly failed withdrawals retrieved successfully",
                    )
                    .await;
                results
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly failed withdrawals for card {} in {}: {}",
                    req.card_number, req.year, e
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly failed withdrawals: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawResponseYearStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseYearStatusFailed::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed yearly withdrawals for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly failed withdrawal records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
