use crate::{
    abstract_trait::merchant::{
        repository::statsbymerchant::method::DynMerchantStatsMethodByMerchantRepository,
        service::statsbymerchant::method::MerchantStatsMethodByMerchantServiceTrait,
    },
    domain::{
        requests::merchant::MonthYearPaymentMethodMerchant,
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

pub struct MerchantStatsMethodByMerchantService {
    method: DynMerchantStatsMethodByMerchantRepository,
}

impl MerchantStatsMethodByMerchantService {
    pub async fn new(method: DynMerchantStatsMethodByMerchantRepository) -> Self {
        Self { method }
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

        let methods = self.method.get_monthly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly payment method data for merchant_id {} in year {}: {e:?}",
                req.merchant_id, req.year, 
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly payment method records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
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

        let methods = self.method.get_yearly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly payment method data for merchant_id {} in year {}: {e:?}",
                req.merchant_id, req.year, 
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly payment method records for merchant_id {}",
            response_data.len(),
            req.merchant_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for merchant_id {} in year {} retrieved successfully",
                req.merchant_id, req.year
            ),
            data: response_data,
        })
    }
}
