use crate::{
    abstract_trait::card::{
        repository::statsbycard::topup::DynCardStatsTopupByCardRepository,
        service::statsbycard::topup::CardStatsTopupByCardServiceTrait,
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

pub struct CardStatsTopupByCardService {
    pub topup: DynCardStatsTopupByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsTopupByCardService {
    pub fn new(topup: DynCardStatsTopupByCardRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            topup,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsTopupByCardServiceTrait for CardStatsTopupByCardService {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📈 Fetching monthly top-up amounts for card: {} (Year: {})",
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
            "get_monthly_topup_amount",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_topup:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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

        let amounts = match self.topup.get_monthly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly top-up records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
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
                    "❌ Failed to retrieve monthly top-up data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly top-up amounts: {}", e),
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
                "Monthly top-up amounts for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly top-up records for card {}",
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
            "💳📊 Fetching yearly top-up amounts for card: {} (Year: {})",
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
            "get_yearly_topup_amount",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_topup:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up amounts in cache for card: {}",
                mask_card_number(&req.card_number)
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

        let amounts = match self.topup.get_yearly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly top-up records for card {}",
                    amounts.len(),
                    mask_card_number(&req.card_number)
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
                    "❌ Failed to retrieve yearly top-up data for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
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

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly top-up amounts for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly top-up records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }
}
