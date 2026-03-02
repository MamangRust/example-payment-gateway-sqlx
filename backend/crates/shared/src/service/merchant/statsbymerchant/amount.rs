use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::amount::DynMerchantStatsAmountByMerchantRepository,
        service::statsbymerchant::amount::MerchantStatsAmountByMerchantServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearAmountMerchant,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
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

pub struct MerchantStatsAmountByMerchantService {
    pub amount: DynMerchantStatsAmountByMerchantRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsAmountByMerchantService {
    pub fn new(
        amount: DynMerchantStatsAmountByMerchantRepository,
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
impl MerchantStatsAmountByMerchantServiceTrait for MerchantStatsAmountByMerchantService {
    async fn find_monthly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError> {
        info!(
            "📅💼 Fetching monthly transaction amounts by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_monthly_amount_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_amount_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transaction records for merchant_id {}",
                    amounts.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transaction amounts by merchant retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transaction amounts by merchant: {:?}",
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
                "Monthly transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transaction records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }

    async fn find_yearly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError> {
        info!(
            "📆💼 Fetching yearly transaction amounts by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_yearly_amount_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_amount_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transaction records for merchant_id {}",
                    amounts.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transaction amounts by merchant retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly transaction amounts by merchant: {:?}",
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
                "Yearly transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transaction records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }
}
