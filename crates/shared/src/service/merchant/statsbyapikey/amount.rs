use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::amount::DynMerchantStatsAmountByApiKeyRepository,
        service::statsbyapikey::amount::MerchantStatsAmountByApiKeyServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearAmountApiKey,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantStatsAmountByApiKeyService {
    amount: DynMerchantStatsAmountByApiKeyRepository,
}

impl MerchantStatsAmountByApiKeyService {
    pub async fn new(amount: DynMerchantStatsAmountByApiKeyRepository) -> Self {
        Self { amount }
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
            req.api_key, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_monthly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly amounts for api_key {} in year {}: {e:?}",
                req.api_key, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transaction records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }

    async fn find_yearly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError> {
        info!(
            "📆💼 Fetching yearly transaction amounts by API key api_key: {} (Year: {})",
            req.api_key, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_yearly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly amounts for api_key {} in year {}: {e:?}",
                req.api_key, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transaction records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }
}
