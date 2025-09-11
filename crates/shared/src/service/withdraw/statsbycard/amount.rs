use crate::{
    abstract_trait::withdraw::{
        repository::statsbycard::amount::DynWithdrawStatsAmountByCardRepository,
        service::statsbycard::amount::WithdrawStatsAmountByCardServiceTrait,
    },
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct WithdrawStatsAmountByCardService {
    amount: DynWithdrawStatsAmountByCardRepository,
}

impl WithdrawStatsAmountByCardService {
    pub async fn new(amount: DynWithdrawStatsAmountByCardRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl WithdrawStatsAmountByCardServiceTrait for WithdrawStatsAmountByCardService {
    async fn get_monthly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_monthly_by_card(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch monthly withdrawal amounts for card {} in {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<WithdrawMonthlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawMonthlyAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly withdrawal records for card {} in {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly withdrawal amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly withdrawal amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_yearly_by_card(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly withdrawal amounts for card {} in {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<WithdrawYearlyAmountResponse> = amounts
            .into_iter()
            .map(WithdrawYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly withdrawal records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly withdrawal amounts for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
