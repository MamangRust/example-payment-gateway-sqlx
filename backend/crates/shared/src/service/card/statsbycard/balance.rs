use crate::{
    abstract_trait::card::{
        repository::statsbycard::balance::DynCardStatsBalanceByCardRepository,
        service::statsbycard::balance::CardStatsBalanceByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
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

pub struct CardStatsBalanceByCardService {
    pub balance: DynCardStatsBalanceByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl CardStatsBalanceByCardService {
    pub fn new(
        balance: DynCardStatsBalanceByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            balance,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardStatsBalanceByCardServiceTrait for CardStatsBalanceByCardService {
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError> {
        info!(
            "💳📅 Fetching monthly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "monthly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_balance:monthly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseMonthBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly balance retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_monthly_balance(req).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} monthly balance records for card {}",
                    balances.len(),
                    mask_card_number(&req.card_number)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly balance retrieved successfully",
                    )
                    .await;
                balances
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly balance for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly balance: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseMonthBalance> = balances
            .into_iter()
            .map(CardResponseMonthBalance::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly balance for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly balance records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }

    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError> {
        info!(
            "💳📆 Fetching yearly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_balance",
            vec![
                KeyValue::new("component", "balance"),
                KeyValue::new("operation", "yearly"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "card_stats_balance:yearly:card:{}:year:{}",
            mask_card_number(&req.card_number),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<CardResponseYearlyBalance>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly balance in cache for card: {}",
                mask_card_number(&req.card_number)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly balance retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let balances = match self.balance.get_yearly_balance(req).await {
            Ok(balances) => {
                info!(
                    "✅ Successfully retrieved {} yearly balance records for card {}",
                    balances.len(),
                    mask_card_number(&req.card_number)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly balance retrieved successfully",
                    )
                    .await;
                balances
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly balance for card {} in year {}: {e:?}",
                    mask_card_number(&req.card_number),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly balance: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<CardResponseYearlyBalance> = balances
            .into_iter()
            .map(CardResponseYearlyBalance::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly balance for card {} in year {} retrieved successfully",
                mask_card_number(&req.card_number),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::hours(1))
            .await;

        info!(
            "✅ Retrieved {} yearly balance records for card {}",
            response.data.len(),
            mask_card_number(&req.card_number)
        );

        Ok(response)
    }
}
