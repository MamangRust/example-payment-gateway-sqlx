use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::totalamount::DynMerchantStatsTotalAmountByApiKeyRepository,
        service::statsbyapikey::totalamount::MerchantStatsTotalAmountByApiKeyServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearTotalAmountApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
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

pub struct MerchantStatsTotalAmountByApiKeyService {
    pub total_amount: DynMerchantStatsTotalAmountByApiKeyRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsTotalAmountByApiKeyService {
    pub fn new(
        total_amount: DynMerchantStatsTotalAmountByApiKeyRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            total_amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByApiKeyServiceTrait for MerchantStatsTotalAmountByApiKeyService {
    async fn find_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError> {
        info!(
            "📅💰 Fetching monthly total transaction amounts by API key (Year: {}) | api_key: {}",
            req.year,
            mask_api_key(&req.api_key)
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_monthly_total_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_total_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_monthly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly total amount records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly total transaction amounts by API key retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly total amounts for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly total transaction amounts by API key: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly total amount records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }

    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!(
            "📆💰 Fetching yearly total transaction amounts by API key (Year: {}) | api_key: {}",
            req.year,
            mask_api_key(&req.api_key)
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_yearly_total_amount_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_total_amount_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_yearly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly total amount records for api_key {}",
                    amounts.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly total transaction amounts by API key retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly total amounts for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly total transaction amounts by API key: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly total amount records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }
}
