use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::method::DynMerchantStatsMethodByApiKeyRepository,
        service::statsbyapikey::method::MerchantStatsMethodByApiKeyServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::MonthYearPaymentMethodApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
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

pub struct MerchantStatsMethodByApiKeyService {
    pub method: DynMerchantStatsMethodByApiKeyRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantStatsMethodByApiKeyService {
    pub fn new(
        method: DynMerchantStatsMethodByApiKeyRepository,
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
impl MerchantStatsMethodByApiKeyServiceTrait for MerchantStatsMethodByApiKeyService {
    async fn find_monthly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError> {
        info!(
            "📅💳 Fetching monthly payment method stats by API key (Year: {}) | api_key: {}",
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
            "find_monthly_method_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "monthly_method_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:monthly_method:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly payment method statistics in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_monthly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} monthly payment method records for api_key {}",
                    methods.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly payment method statistics by API key retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly payment method data for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly payment method statistics by API key: {:?}",
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
                "Monthly payment method statistics for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly payment method records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!(
            "📆💳 Fetching yearly payment method stats by API key (Year: {}) | api_key: {}",
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
            "find_yearly_method_by_api_key",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "yearly_method_by_api_key"),
                KeyValue::new("api_key", mask_api_key(&req.api_key)),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "merchant:yearly_method:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly payment method statistics in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let methods = match self.method.get_yearly_method(req).await {
            Ok(methods) => {
                info!(
                    "✅ Successfully retrieved {} yearly payment method records for api_key {}",
                    methods.len(),
                    mask_api_key(&req.api_key)
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly payment method statistics by API key retrieved successfully",
                    )
                    .await;
                methods
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly payment method data for api_key '{}' in year {}: {e:?}",
                    mask_api_key(&req.api_key),
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly payment method statistics by API key: {:?}",
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
                "Yearly payment method statistics for api_key {} in year {} retrieved successfully",
                mask_api_key(&req.api_key),
                req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly payment method records for api_key {}",
            response.data.len(),
            mask_api_key(&req.api_key)
        );

        Ok(response)
    }
}
