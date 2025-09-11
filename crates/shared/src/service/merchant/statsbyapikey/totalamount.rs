use crate::{
    abstract_trait::merchant::{
        repository::statsbyapikey::totalamount::DynMerchantStatsTotalAmountByApiKeyRepository,
        service::statsbyapikey::totalamount::MerchantStatsTotalAmountByApiKeyServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearTotalAmountApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantStatsTotalAmountByApiKeyService {
    total_amount: DynMerchantStatsTotalAmountByApiKeyRepository,
}

impl MerchantStatsTotalAmountByApiKeyService {
    pub async fn new(total_amount: DynMerchantStatsTotalAmountByApiKeyRepository) -> Self {
        Self { total_amount }
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
            req.year, req.api_key
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .total_amount
            .get_monthly_total_amount(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly total amounts for api_key '{}' in year {}: {e:?}",
                    req.api_key, req.year, 
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly total amount records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }

    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!(
            "📆💰 Fetching yearly total transaction amounts by API key (Year: {}) | api_key: {}",
            req.year, req.api_key
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .total_amount
            .get_yearly_total_amount(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve yearly total amounts for api_key '{}' in year {}: {e:?}",
                    req.api_key, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly total amount records for api_key {}",
            response_data.len(),
            req.api_key
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for api_key {} in year {} retrieved successfully",
                req.api_key, req.year
            ),
            data: response_data,
        })
    }
}
