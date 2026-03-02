use crate::{
    abstract_trait::withdraw::{
        repository::statsbycard::amount::DynWithdrawStatsAmountByCardRepository,
        service::statsbycard::amount::WithdrawStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
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

pub struct WithdrawStatsAmountByCardService {
    pub amount: DynWithdrawStatsAmountByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawStatsAmountByCardService {
    pub fn new(
        amount: DynWithdrawStatsAmountByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl WithdrawStatsAmountByCardServiceTrait for WithdrawStatsAmountByCardService {
    async fn get_monthly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_withdrawal_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "monthly_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "withdrawal:monthly_by_card:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly withdrawal amounts in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly withdrawal amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_by_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly withdrawal records for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly withdrawal amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly withdrawal amounts for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly withdrawal amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawMonthlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawMonthlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly withdrawal amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly withdrawal records for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
    }

    async fn get_yearly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_withdrawal_by_card",
            vec![
                KeyValue::new("component", "withdrawal"),
                KeyValue::new("operation", "yearly_by_card"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "withdrawal:yearly_by_card:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<WithdrawYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly withdrawal amounts in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly withdrawal amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_by_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly withdrawal records for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly withdrawal amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly withdrawal amounts for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly withdrawal amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<WithdrawYearlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly withdrawal amounts for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly withdrawal records for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
