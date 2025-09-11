use crate::{
    abstract_trait::merchant::{
        repository::stats::method::DynMerchantStatsMethodRepository,
        service::stats::method::MerchantStatsMethodServiceTrait,
    },
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantStatsMethodService {
    method: DynMerchantStatsMethodRepository,
}

impl MerchantStatsMethodService {
    pub async fn new(method: DynMerchantStatsMethodRepository) -> Self {
        Self { method }
    }
}

#[async_trait]
impl MerchantStatsMethodServiceTrait for MerchantStatsMethodService {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError> {
        info!("📅💳 Fetching monthly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_monthly_method(year).await.map_err(|e| {
            error!("❌ Failed to retrieve monthly payment method data for year {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseMonthlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly payment method records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError> {
        info!("📆💳 Fetching yearly payment method statistics for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_yearly_method(year).await.map_err(|e| {
            error!("❌ Failed to retrieve yearly payment method data for year {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyPaymentMethod> = methods
            .into_iter()
            .map(MerchantResponseYearlyPaymentMethod::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly payment method records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly payment method statistics for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }
}
