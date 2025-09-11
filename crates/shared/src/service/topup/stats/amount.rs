use crate::{
    abstract_trait::topup::{
        repository::stats::amount::DynTopupStatsAmountRepository,
        service::stats::amount::TopupStatsAmountServiceTrait,
    },
    domain::responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    errors::ServiceError,
};
use async_trait::async_trait;
use tracing::{error, info};

pub struct TopupStatsAmountService {
    amount: DynTopupStatsAmountRepository,
}

impl TopupStatsAmountService {
    pub async fn new(amount: DynTopupStatsAmountRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TopupStatsAmountServiceTrait for TopupStatsAmountService {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError> {
        info!("📅💸 Fetching monthly top-up amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .amount
            .get_monthly_topup_amounts(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve monthly top-up amounts for year {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupMonthAmountResponse> = amounts
            .into_iter()
            .map(TopupMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly top-up records for year {}",
            response_data.len(),
            year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError> {
        info!("📆💸 Fetching yearly top-up amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .amount
            .get_yearly_topup_amounts(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve yearly top-up amounts for year {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupYearlyAmountResponse> = amounts
            .into_iter()
            .map(TopupYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly top-up records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
