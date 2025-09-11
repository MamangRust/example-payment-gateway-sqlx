use crate::{
    abstract_trait::card::{
        repository::stats::balance::DynCardStatsBalanceRepository,
        service::stats::balance::CardStatsBalanceServiceTrait,
    },
    domain::responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardStatsBalanceService {
    balance: DynCardStatsBalanceRepository,
}

impl CardStatsBalanceService {
    pub async fn new(balance: DynCardStatsBalanceRepository) -> Self {
        Self { balance }
    }
}

#[async_trait]
impl CardStatsBalanceServiceTrait for CardStatsBalanceService {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError> {
        info!("📅 Fetching monthly balance statistics for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let balances = self.balance.get_monthly_balance(year).await.map_err(|e| {
            error!("🗄️ Failed to retrieve monthly balance from repository for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthBalance> = balances
            .into_iter()
            .map(CardResponseMonthBalance::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} monthly balance records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly balance for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError> {
        info!("📆 Fetching yearly balance statistics for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let balances = self.balance.get_yearly_balance(year).await.map_err(|e| {
            error!("🗄️ Failed to retrieve yearly balance from repository for year {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearlyBalance> = balances
            .into_iter()
            .map(CardResponseYearlyBalance::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} yearly balance records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly balance for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
