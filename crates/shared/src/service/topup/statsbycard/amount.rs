use crate::{
    abstract_trait::topup::{
        repository::statsbycard::amount::DynTopupStatsAmountByCardRepository,
        service::statsbycard::amount::TopupStatsAmountByCardServiceTrait,
    },
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TopupStatsAmountByCardService {
    amount: DynTopupStatsAmountByCardRepository,
}

impl TopupStatsAmountByCardService {
    pub async fn new(amount: DynTopupStatsAmountByCardRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TopupStatsAmountByCardServiceTrait for TopupStatsAmountByCardService {
    async fn get_monthly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError> {
        info!(
            "📊 Fetching monthly top-up amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_monthly_amounts(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch monthly amounts for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupMonthAmountResponse> = amounts
            .into_iter()
            .map(TopupMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly top-up records for card {} in year {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly top-up amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError> {
        info!(
            "📅💰 Fetching yearly top-up amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_yearly_amounts(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly amounts for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupYearlyAmountResponse> = amounts
            .into_iter()
            .map(TopupYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly top-up records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly top-up amounts for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
