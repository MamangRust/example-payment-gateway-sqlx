use crate::{
    abstract_trait::card::{
        repository::statsbycard::transfer::DynCardStatsTransferByCardRepository,
        service::statsbycard::transfer::CardStatsTransferByCardServiceTrait,
    },
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct CardStatsTransferByCardService {
    transfer: DynCardStatsTransferByCardRepository,
}

impl CardStatsTransferByCardService {
    pub async fn new(transfer: DynCardStatsTransferByCardRepository) -> Self {
        Self { transfer }
    }
}

#[async_trait]
impl CardStatsTransferByCardServiceTrait for CardStatsTransferByCardService {
    async fn get_monthly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📤 Fetching monthly transfer amounts (sent) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .transfer
            .get_monthly_amount_sender(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly 'sent' transfer data for card {} in year {}: {e:?}",
                    req.card_number, req.year, 
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly 'sent' transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (sent) for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📈📤 Fetching yearly transfer amounts (sent) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .transfer
            .get_yearly_amount_sender(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve yearly 'sent' transfer data for card {} in year {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly 'sent' transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (sent) for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_monthly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📥 Fetching monthly transfer amounts (received) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.transfer.get_monthly_amount_receiver(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly 'received' transfer data for card {} in year {}: {e:?}",
                req.card_number, req.year, 
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly 'received' transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (received) for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📈📥 Fetching yearly transfer amounts (received) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.transfer.get_yearly_amount_receiver(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly 'received' transfer data for card {} in year {}: {e:?}",
                req.card_number, req.year, 
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly 'received' transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (received) for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
