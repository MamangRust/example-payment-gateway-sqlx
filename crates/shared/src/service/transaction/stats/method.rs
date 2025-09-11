use crate::{
    abstract_trait::transaction::{
        repository::stats::method::DynTransactionStatsMethodRepository,
        service::stats::method::TransactionStatsMethodServiceTrait,
    },
    domain::responses::{
        ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TransactionStatsMethodService {
    method: DynTransactionStatsMethodRepository,
}

impl TransactionStatsMethodService {
    pub async fn new(method: DynTransactionStatsMethodRepository) -> Self {
        Self { method }
    }
}

#[async_trait]
impl TransactionStatsMethodServiceTrait for TransactionStatsMethodService {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError> {
        info!("📊 Fetching monthly transaction methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_monthly_method(year).await.map_err(|e| {
            error!("❌ Failed to fetch monthly transaction methods for {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionMonthMethodResponse> = methods
            .into_iter()
            .map(TransactionMonthMethodResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly transaction method records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly transaction methods for year {year} retrieved successfully",),
            data: response_data,
        })
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError> {
        info!("📅📊 Fetching yearly transaction methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_yearly_method(year).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly transaction methods for {year}: {e:?}"
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionYearMethodResponse> = methods
            .into_iter()
            .map(TransactionYearMethodResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly transaction method records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction methods for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }
}
