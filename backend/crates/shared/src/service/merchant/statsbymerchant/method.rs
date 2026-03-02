use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::method::DynMerchantStatsMethodByMerchantRepository,
        service::statsbymerchant::method::MerchantStatsMethodByMerchantServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearPaymentMethodMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
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

pub struct MerchantStatsMethodByMerchantService {
    pub method: DynMerchantStatsMethodByMerchantRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsMethodByMerchantService {
    pub fn new(
        method: DynMerchantStatsMethodByMerchantRepository,
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
impl MerchantStatsMethodByMerchantServiceTrait for MerchantStatsMethodByMerchantService {
    async fn find_monthly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError> {
        info!(
            "📅💳 Fetching monthly payment method stats by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_monthly_method_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_method_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_method:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly payment method statistics in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly payment method records for merchant_id {}",
                    methods.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly payment method statistics by merchant retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly payment method data for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!(
                        "Failed to retrieve monthly payment method statistics by merchant: {:?}",
                        e
                    ),
                )
                .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly payment method records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!(
            "📆💳 Fetching yearly payment method stats by merchant (merchant_id: {}, year: {})",
            req.merchant_id, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_yearly_method_by_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_method_by_merchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_method:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly payment method statistics in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly payment method records for merchant_id {}",
                    methods.len(),
                    req.merchant_id
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly payment method statistics by merchant retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly payment method data for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly payment method statistics by merchant: {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly payment method records for merchant_id {}",
            response.data.len(),
            req.merchant_id
        );

        Ok(response)
    }
}
