use crate::{
    abstract_trait::merchant::{
        repository::stats::amount::DynMerchantStatsAmountRepository,
        service::stats::amount::MerchantStatsAmountServiceTrait,
    },
    domain::responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantStatsAmountService {
    amount: DynMerchantStatsAmountRepository,
}

impl MerchantStatsAmountService {
    pub async fn new(amount: DynMerchantStatsAmountRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl MerchantStatsAmountServiceTrait for MerchantStatsAmountService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError> {
        info!(
            "📅📊 Fetching monthly transaction amounts for merchant (Year: {})",
            year
        );

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_monthly_amount(year).await.map_err(|e| {
            error!("❌ Failed to retrieve monthly amounts for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseMonthlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseMonthlyAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} monthly merchant records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly merchant amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError> {
        info!("📆📈 Fetching yearly transaction amounts for merchant (Year: {year})");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_yearly_amount(year).await.map_err(|e| {
            error!("❌ Failed to retrieve yearly amounts for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<MerchantResponseYearlyAmount> = amounts
            .into_iter()
            .map(MerchantResponseYearlyAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} yearly merchant records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly merchant amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
