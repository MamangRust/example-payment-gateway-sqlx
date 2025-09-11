use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::totalamount::DynMerchantStatsTotalAmountByMerchantRepository,
        service::statsbymerchant::totalamount::MerchantStatsTotalAmountByMerchantServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearTotalAmountMerchant,
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

pub struct MerchantStatsTotalAmountByMerchantService {
    total_amount: DynMerchantStatsTotalAmountByMerchantRepository,
}

impl MerchantStatsTotalAmountByMerchantService {
    pub async fn new(total_amount: DynMerchantStatsTotalAmountByMerchantRepository) -> Self {
        Self { total_amount }
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

        let amounts = self
            .total_amount
            .get_monthly_total_amount(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly total amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year, 
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly total amount records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
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

        let amounts = self
            .total_amount
            .get_yearly_total_amount(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve yearly total amounts for merchant_id {} in year {}: {e:?}",
                    req.merchant_id, req.year, 
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly total amount records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
    }
}
