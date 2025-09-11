use crate::{
    abstract_trait::card::{
        repository::statsbycard::transaction::DynCardStatsTransactionByCardRepository,
        service::statsbycard::transaction::CardStatsTransactionByCardServiceTrait,
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

pub struct CardStatsTransactionByCardService {
    transaction: DynCardStatsTransactionByCardRepository,
}

impl CardStatsTransactionByCardService {
    pub async fn new(transaction: DynCardStatsTransactionByCardRepository) -> Self {
        Self { transaction }
    }
}

#[async_trait]
impl CardStatsTransactionByCardServiceTrait for CardStatsTransactionByCardService {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly transaction amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .transaction
            .get_monthly_amount(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly transaction data for card {} in year {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transaction records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for card {} in year {} retrieved successfully",
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
            "💳📈 Fetching yearly transaction amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.transaction.get_yearly_amount(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly transaction data for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transaction records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
