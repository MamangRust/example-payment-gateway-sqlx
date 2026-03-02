use crate::{
    abstract_trait::topup::{
        repository::statsbycard::amount::DynTopupStatsAmountByCardRepository,
        service::statsbycard::amount::TopupStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
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

pub struct TopupStatsAmountByCardService {
    pub amount: DynTopupStatsAmountByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupStatsAmountByCardService {
    pub fn new(
        amount: DynTopupStatsAmountByCardRepository,
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
impl TopupStatsAmountByCardServiceTrait for TopupStatsAmountByCardService {
    async fn get_monthly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📊 Fetching monthly top-up amounts for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amounts",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_amounts"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:monthly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up amounts in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly top-up amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly top-up records for card {}",
                    amounts.len(),
                    masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly top-up amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly top-up amounts for card {} in year {}: {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly top-up amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupMonthAmountResponse> = amounts
            .into_iter()
            .map(TopupMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly top-up amounts for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly top-up records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }

    async fn get_yearly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📈 Fetching yearly top-up amounts for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amounts",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_amounts"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:yearly_amounts:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up amounts in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly top-up amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly top-up records for card {}",
                    amounts.len(),
                    masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly top-up amounts retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly top-up amounts for card {} in year {}: {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly top-up amounts: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupYearlyAmountResponse> = amounts
            .into_iter()
            .map(TopupYearlyAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly top-up amounts for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly top-up records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
