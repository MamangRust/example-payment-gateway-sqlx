use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::totalamount::DynMerchantStatsTotalAmountByMerchantRepository,
        service::statsbymerchant::totalamount::MerchantStatsTotalAmountByMerchantServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearTotalAmountMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
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

pub struct MerchantStatsTotalAmountByMerchantService {
    pub total_amount: DynMerchantStatsTotalAmountByMerchantRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsTotalAmountByMerchantService {
    pub fn new(
        total_amount: DynMerchantStatsTotalAmountByMerchantRepository,
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
impl MerchantStatsTotalAmountByMerchantServiceTrait for MerchantStatsTotalAmountByMerchantService {
    async fn find_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError> {
        info!(
            "📅💰 Fetching monthly total transaction amounts by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_monthly_total_amount_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_total_amount_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_total_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_monthly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly total amount records for merchant_id {}",
                    amounts.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly total transaction amounts by merchant retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly total amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly total transaction amounts by merchant: {:?}",
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
                "Monthly total transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly total amount records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }

    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!(
            "📆💰 Fetching yearly total transaction amounts by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_yearly_total_amount_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_total_amount_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_total_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly total transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.total_amount.get_yearly_total_amount(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly total amount records for merchant_id {}",
                    amounts.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly total transaction amounts by merchant retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly total amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly total transaction amounts by merchant: {:?}",
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
                "Yearly total transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly total amount records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }
}
