use crate::{
    abstract_trait::card::{
        repository::statsbycard::withdraw::DynCardStatsWithdrawByCardRepository,
        service::statsbycard::withdraw::CardStatsWithdrawByCardServiceTrait,
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

pub struct CardStatsWithdrawByCardService {
    withdraw: DynCardStatsWithdrawByCardRepository,
}

impl CardStatsWithdrawByCardService {
    pub async fn new(withdraw: DynCardStatsWithdrawByCardRepository) -> Self {
        Self { withdraw }
    }
}

#[async_trait]
impl CardStatsWithdrawByCardServiceTrait for CardStatsWithdrawByCardService {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "🏧💳 Fetching monthly withdraw amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.withdraw.get_monthly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly withdraw data for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly withdraw records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly withdraw amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!(
            "📉🏧 Fetching yearly withdraw amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.withdraw.get_yearly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly withdraw data for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly withdraw records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly withdraw amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
