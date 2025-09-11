use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::method::DynMerchantStatsMethodByApiKeyRepository,
        service::statsbyapikey::method::MerchantStatsMethodByApiKeyServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearPaymentMethodApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantStatsMethodByApiKeyService {
    method: DynMerchantStatsMethodByApiKeyRepository,
}

impl MerchantStatsMethodByApiKeyService {
    pub async fn new(method: DynMerchantStatsMethodByApiKeyRepository) -> Self {
        Self { method }
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
            req.year, req.api_key
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_monthly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly payment method data for api_key '{}' in year {}: {e:?}",
                req.api_key, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly payment method records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!(
            "📆💳 Fetching yearly payment method stats by API key (Year: {}) | api_key: {}",
            req.year, req.api_key
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_yearly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly payment method data for api_key '{}' in year {}: {e:?}",
                req.api_key, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly payment method records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }
}
