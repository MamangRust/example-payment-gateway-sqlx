use crate::{
    abstract_trait::withdraw::{
        repository::statsbycard::status::DynWithdrawStatsStatusByCardRepository,
        service::statsbycard::status::WithdrawStatsStatusByCardServiceTrait,
    },
    domain::{
        requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
        responses::{
            ApiResponse, WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
            WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct WithdrawStatsStatusByCardService {
    status: DynWithdrawStatsStatusByCardRepository,
}

impl WithdrawStatsStatusByCardService {
    pub async fn new(status: DynWithdrawStatsStatusByCardRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl WithdrawStatsStatusByCardServiceTrait for WithdrawStatsStatusByCardService {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful monthly withdrawals for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_success_by_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch successful monthly withdrawals for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseMonthStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful monthly withdrawal records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful withdrawals for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful withdrawals for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_success_by_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly successful withdrawals for card {} in {}: {}",
                    req.card_number, req.year, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseYearStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful withdrawal records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly withdrawals for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed monthly withdrawals for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_failed_by_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch failed monthly withdrawals for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseMonthStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed monthly withdrawal records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed withdrawals for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed withdrawals for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_failed_by_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly failed withdrawals for card {} in {}: {}",
                    req.card_number, req.year, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseYearStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed withdrawal records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed yearly withdrawals for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
