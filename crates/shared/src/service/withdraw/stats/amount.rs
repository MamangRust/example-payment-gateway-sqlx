use crate::{
    abstract_trait::withdraw::{
        repository::stats::amount::DynWithdrawStatsAmountRepository,
        service::stats::amount::WithdrawStatsAmountServiceTrait,
    },
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct WithdrawStatsAmountService {
    amount: DynWithdrawStatsAmountRepository,
}

impl WithdrawStatsAmountService {
    pub async fn new(amount: DynWithdrawStatsAmountRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl WithdrawStatsAmountServiceTrait for WithdrawStatsAmountService {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_monthly_withdraws(year).await.map_err(|e| {
            error!("❌ Failed to fetch monthly withdrawal amounts for {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<WithdrawMonthlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawMonthlyAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly withdrawal records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly withdrawal amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }

    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly withdrawal amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_yearly_withdraws(year).await.map_err(|e| {
            error!("❌ Failed to fetch yearly withdrawal amounts for {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<WithdrawYearlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly withdrawal records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly withdrawal amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }
}
