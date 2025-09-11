use crate::{
    abstract_trait::card::{
        repository::stats::transfer::DynCardStatsTransferRepository,
        service::stats::transfer::CardStatsTransferServiceTrait,
    },
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardStatsTransferService {
    transfer: DynCardStatsTransferRepository,
}

impl CardStatsTransferService {
    pub async fn new(transfer: DynCardStatsTransferRepository) -> Self {
        Self { transfer }
    }
}

#[async_trait]
impl CardStatsTransferServiceTrait for CardStatsTransferService {
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("📤 Fetching monthly transfer amounts (sent) for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transfer
            .get_monthly_amount_sender(year)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly transfer (sender) data for year {year}: {e:?}"
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly 'sent' transfer records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (sent) for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📈📤 Fetching yearly transfer amounts (sent) for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transfer
            .get_yearly_amount_sender(year)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve yearly transfer (sender) data for year {year}: {e:?}"
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly 'sent' transfer records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (sent) for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }

    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "📥 Fetching monthly transfer amounts (received) for year: {}",
            year
        );

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transfer
            .get_monthly_amount_receiver(year)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly transfer (receiver) data for year {year}: {e:?}",
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly 'received' transfer records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (received) for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📈📥 Fetching yearly transfer amounts (received) for year: {year}",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self
            .transfer
            .get_yearly_amount_receiver(year)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve yearly transfer (receiver) data for year {}: {}",
                    year, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly 'received' transfer records for year {}",
            response_data.len(),
            year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (received) for year {year} retrieved successfully"
            ),
            data: response_data,
        })
    }
}
