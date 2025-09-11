use crate::{
    abstract_trait::withdraw::{
        repository::stats::status::DynWithdrawStatsStatusRepository,
        service::stats::status::WithdrawStatsStatusServiceTrait,
    },
    domain::{
        requests::withdraw::MonthStatusWithdraw,
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

pub struct WithdrawStatsStatusService {
    status: DynWithdrawStatsStatusRepository,
}

impl WithdrawStatsStatusService {
    pub async fn new(status: DynWithdrawStatsStatusRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl WithdrawStatsStatusServiceTrait for WithdrawStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful withdrawals for month: {}-{}",
            req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_success(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch successful withdrawals for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseMonthStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful withdrawal records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful withdrawals for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError> {
        info!("📅✅ Fetching yearly successful withdrawals for year: {year}",);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {}", msg);
            return Err(ServiceError::Custom(msg));
        }

        let results = self
            .status
            .get_yearly_status_success(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch yearly successful withdrawals for {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseYearStatusSuccess> = results
            .into_iter()
            .map(WithdrawResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful withdrawal records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly withdrawals for year {year}",
            ),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed withdrawals for month: {}-{}",
            req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_failed(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch failed withdrawals for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseMonthStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed withdrawal records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed withdrawals for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError> {
        info!("📅❌ Fetching yearly failed withdrawals for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let results = self
            .status
            .get_yearly_status_failed(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch yearly failed withdrawals for {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<WithdrawResponseYearStatusFailed> = results
            .into_iter()
            .map(WithdrawResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed withdrawal records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed yearly withdrawals for year {year}",),
            data: response_data,
        })
    }
}
