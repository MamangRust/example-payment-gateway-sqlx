use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::amount::DynMerchantStatsAmountByApiKeyRepository,
        service::statsbyapikey::amount::MerchantStatsAmountByApiKeyServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearAmountApiKey,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
    utils::mask_api_key,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantStatsAmountByApiKeyService {
    pub amount: DynMerchantStatsAmountByApiKeyRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsAmountByApiKeyService {
    pub fn new(
        amount: DynMerchantStatsAmountByApiKeyRepository,
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
impl MerchantStatsAmountByApiKeyServiceTrait for MerchantStatsAmountByApiKeyService {
    async fn find_monthly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError> {
        info!(
            "📅💼 Fetching monthly transaction amounts by API key for api_key: {} (Year: {})",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_monthly_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transaction amounts by API key retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly amounts for api_key {} in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transaction amounts by API key: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }

    async fn find_yearly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError> {
        info!(
            "📆💼 Fetching yearly transaction amounts by API key api_key: {} (Year: {})",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_yearly_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transaction amounts by API key retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly amounts for api_key {} in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly transaction amounts by API key: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }
}
