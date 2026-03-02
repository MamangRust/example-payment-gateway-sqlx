use crate::{
    abstract_trait::card::{
        repository::statsbycard::withdraw::DynCardStatsWithdrawByCardRepository,
        service::statsbycard::withdraw::CardStatsWithdrawByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct CardStatsWithdrawByCardService {
    pub withdraw: DynCardStatsWithdrawByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsWithdrawByCardService {
    pub fn new(
        withdraw: DynCardStatsWithdrawByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            withdraw,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsWithdrawByCardServiceTrait for CardStatsWithdrawByCardService {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "🏧💳 Fetching monthly withdraw amounts for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_withdraw_amount",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_withdraw:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly withdraw amounts in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly withdraw amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.withdraw.get_monthly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly withdraw records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly withdraw amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly withdraw data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly withdraw amounts: {:?}", e),
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
                "Monthly withdraw amounts for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly withdraw records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📉🏧 Fetching yearly withdraw amounts for card: {} (Year: {})",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_withdraw_amount",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_withdraw:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly withdraw amounts in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly withdraw amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.withdraw.get_yearly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly withdraw records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly withdraw amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly withdraw data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly withdraw amounts: {:?}", e),
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
                "Yearly withdraw amounts for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly withdraw records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }
}
