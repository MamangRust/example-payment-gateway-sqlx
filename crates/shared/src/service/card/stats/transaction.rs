use crate::{
    abstract_trait::card::{
        repository::stats::transaction::DynCardStatsTransactionRepository,
        service::stats::transaction::CardStatsTransactionServiceTrait,
    },
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardStatsTransactionService {
    transaction: DynCardStatsTransactionRepository,
}

impl CardStatsTransactionService {
    pub async fn new(transaction: DynCardStatsTransactionRepository) -> Self {
        Self { transaction }
    }
}

#[async_trait]
impl CardStatsTransactionServiceTrait for CardStatsTransactionService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("📊 Fetching monthly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transaction
            .get_monthly_amount(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve monthly transaction data for year {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} monthly transaction records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly transaction amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📈 Fetching yearly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transaction
            .get_yearly_amount(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve yearly transaction data for year {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} yearly transaction records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly transaction amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
