use crate::{
    abstract_trait::merchant::{
        repository::stats::totalamount::DynMerchantStatsTotalAmountRepository,
        service::stats::totalamount::MerchantStatsTotalAmountServiceTrait,
    },
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantStatsTotalAmountService {
    method: DynMerchantStatsTotalAmountRepository,
}

impl MerchantStatsTotalAmountService {
    pub async fn new(method: DynMerchantStatsTotalAmountRepository) -> Self {
        Self { method }
    }
}
#[async_trait]
impl MerchantStatsTotalAmountServiceTrait for MerchantStatsTotalAmountService {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError> {
        info!("📅💰 Fetching monthly total transaction amounts for merchants (Year: {year})",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .method
            .get_monthly_total_amount(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve monthly total amounts for year {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseMonthlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly total amount records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total transaction amounts for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }

    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError> {
        info!("📆💰 Fetching yearly total transaction amounts for merchants (Year: {year})",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .method
            .get_yearly_total_amount(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve yearly total amounts for year {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<MerchantResponseYearlyTotalAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyTotalAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly total amount records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly total transaction amounts for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }
}
