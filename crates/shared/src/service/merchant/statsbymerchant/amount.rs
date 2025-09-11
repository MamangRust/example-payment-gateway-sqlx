use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::amount::DynMerchantStatsAmountByMerchantRepository,
        service::statsbymerchant::amount::MerchantStatsAmountByMerchantServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearAmountMerchant,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantStatsAmountByMerchantService {
    amount: DynMerchantStatsAmountByMerchantRepository,
}

impl MerchantStatsAmountByMerchantService {
    pub async fn new(amount: DynMerchantStatsAmountByMerchantRepository) -> Self {
        Self { amount }
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

        let amounts = self.amount.get_monthly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly amounts for merchant_id {} in year {}: {}",
                req.merchant_id, req.year, e
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transaction records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
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

        let amounts = self.amount.get_yearly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly amounts for merchant_id {} in year {}: {e:?}",
                req.merchant_id, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transaction records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
    }
}
