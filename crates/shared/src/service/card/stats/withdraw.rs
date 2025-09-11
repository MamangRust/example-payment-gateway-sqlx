use crate::{
    abstract_trait::card::{
        repository::stats::withdraw::DynCardStatsWithdrawRepository,
        service::stats::withdraw::CardStatsWithdrawServiceTrait,
    },
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardStatsWithdrawService {
    withdraw: DynCardStatsWithdrawRepository,
}

impl CardStatsWithdrawService {
    pub async fn new(withdraw: DynCardStatsWithdrawRepository) -> Self {
        Self { withdraw }
    }
}

#[async_trait]
impl CardStatsWithdrawServiceTrait for CardStatsWithdrawService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("🏧 Fetching monthly withdrawal amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.withdraw.get_monthly_amount(year).await.map_err(|e| {
            error!("❌ Failed to retrieve monthly withdrawal data for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} monthly withdrawal records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📉🏧 Fetching yearly withdrawal amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.withdraw.get_yearly_amount(year).await.map_err(|e| {
            error!("❌ Failed to retrieve yearly withdrawal data for year {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} yearly withdrawal records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly withdrawal amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
