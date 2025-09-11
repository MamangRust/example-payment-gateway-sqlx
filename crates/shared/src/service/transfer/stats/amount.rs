use crate::{
    abstract_trait::transfer::{
        repository::stats::amount::DynTransferStatsAmountRepository,
        service::stats::amount::TransferStatsAmountServiceTrait,
    },
    domain::responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TransferStatsAmountService {
    amount: DynTransferStatsAmountRepository,
}

impl TransferStatsAmountService {
    pub async fn new(amount: DynTransferStatsAmountRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TransferStatsAmountServiceTrait for TransferStatsAmountService {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!("📊 Fetching monthly transfer amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_monthly_amounts(year).await.map_err(|e| {
            error!("❌ Failed to fetch monthly transfer amounts for {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly transfer records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly transfer amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!("📅💰 Fetching yearly transfer amounts for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.amount.get_yearly_amounts(year).await.map_err(|e| {
            error!("❌ Failed to fetch yearly transfer amounts for {year}: {e:?}",);
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly transfer records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly transfer amounts for year {year} retrieved successfully",),
            data: response_data,
        })
    }
}
