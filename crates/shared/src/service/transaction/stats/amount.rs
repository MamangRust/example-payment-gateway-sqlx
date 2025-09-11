use crate::{
    abstract_trait::transaction::{
        repository::stats::amount::DynTransactionStatsAmountRepository,
        service::stats::amount::TransactionStatsAmountServiceTrait,
    },
    domain::responses::{
        ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TransactionStatsAmountService {
    amount: DynTransactionStatsAmountRepository,
}

impl TransactionStatsAmountService {
    pub async fn new(amount: DynTransactionStatsAmountRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TransactionStatsAmountServiceTrait for TransactionStatsAmountService {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_monthly_amounts(year).await.map_err(|e| {
            error!("❌ Failed to fetch monthly transaction amounts for {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionMonthAmountResponse> = amounts
            .into_iter()
            .map(TransactionMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly transaction records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly transaction amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly transaction amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_yearly_amounts(year).await.map_err(|e| {
            error!("❌ Failed to fetch yearly transaction amounts for {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionYearlyAmountResponse> = amounts
            .into_iter()
            .map(TransactionYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly transaction records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly transaction amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }
}
