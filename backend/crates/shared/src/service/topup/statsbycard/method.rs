use crate::{
    abstract_trait::topup::{
        repository::statsbycard::method::DynTopupStatsMethodByCardRepository,
        service::statsbycard::method::TopupStatsMethodByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
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

pub struct TopupStatsMethodByCardService {
    pub method: DynTopupStatsMethodByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TopupStatsMethodByCardService {
    pub fn new(
        method: DynTopupStatsMethodByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            method,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TopupStatsMethodByCardServiceTrait for TopupStatsMethodByCardService {
    async fn get_monthly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "💳📊 Fetching monthly top-up methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_methods",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "monthly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:monthly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupMonthMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly top-up methods in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly top-up methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_methods(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly top-up method records for card {}",
                    methods.len(),
                    masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly top-up methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly top-up methods for card {} in year {}: {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve monthly top-up methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupMonthMethodResponse> = methods
            .into_iter()
            .map(TopupMonthMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly top-up methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly top-up method records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }

    async fn get_yearly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "📅💳 Fetching yearly top-up methods for card: {} (Year: {})",
            masked_card, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_methods",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "yearly_methods"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "topup:yearly_methods:card:{}:year:{}",
            masked_card, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TopupYearlyMethodResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly top-up methods in cache for card: {}",
                masked_card
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly top-up methods retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_methods(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly top-up method records for card {}",
                    methods.len(),
                    masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly top-up methods retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly top-up methods for card {} in year {}: {e:?}",
                    masked_card, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to retrieve yearly top-up methods: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TopupYearlyMethodResponse> = methods
            .into_iter()
            .map(TopupYearlyMethodResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly top-up methods for card {} in year {} retrieved successfully",
                masked_card, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly top-up method records for card {}",
            response.data.len(),
            masked_card
        );

        Ok(response)
    }
}
