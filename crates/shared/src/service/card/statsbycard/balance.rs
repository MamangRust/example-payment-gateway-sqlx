use crate::{
    abstract_trait::card::{
        repository::statsbycard::balance::DynCardStatsBalanceByCardRepository,
        service::statsbycard::balance::CardStatsBalanceByCardServiceTrait,
    },
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct CardStatsBalanceByCardService {
    balance: DynCardStatsBalanceByCardRepository,
}

impl CardStatsBalanceByCardService {
    pub async fn new(balance: DynCardStatsBalanceByCardRepository) -> Self {
        Self { balance }
    }
}

#[async_trait]
impl CardStatsBalanceByCardServiceTrait for CardStatsBalanceByCardService {
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError> {
        info!(
            "💳📅 Fetching monthly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let balances = self.balance.get_monthly_balance(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve monthly balance for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthBalance> = balances
            .into_iter()
            .map(CardResponseMonthBalance::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly balance records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly balance for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError> {
        info!(
            "💳📆 Fetching yearly balance for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let balances = self.balance.get_yearly_balance(req).await.map_err(|e| {
            error!(
                "❌ Failed to retrieve yearly balance for card {} in year {}: {e:?}",
                req.card_number, req.year
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearlyBalance> = balances
            .into_iter()
            .map(CardResponseYearlyBalance::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly balance records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly balance for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
