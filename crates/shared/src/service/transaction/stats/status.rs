use crate::{
    abstract_trait::transaction::{
        repository::stats::status::DynTransactionStatsStatusRepository,
        service::stats::status::TransactionStatsStatusServiceTrait,
    },
    domain::{
        requests::transaction::MonthStatusTransaction,
        responses::{
            ApiResponse, TransactionResponseMonthStatusFailed,
            TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
            TransactionResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionStatsStatusService {
    status: DynTransactionStatsStatusRepository,
}

impl TransactionStatsStatusService {
    pub async fn new(status: DynTransactionStatsStatusRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TransactionStatsStatusServiceTrait for TransactionStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful transactions for month: {}-{}",
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
                    "❌ Failed to fetch successful transactions for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful transaction records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transactions for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError> {
        info!("📅✅ Fetching yearly successful transactions for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let results = self
            .status
            .get_yearly_status_success(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch yearly successful transactions for {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful transaction records for {}",
            response_data.len(),
            year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful transactions for year {year}"),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed transactions for month: {}-{}",
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
                    "❌ Failed to fetch failed transactions for {}-{}: {}",
                    req.year, req.month, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransactionResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed transaction records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transactions for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed transactions for year: {}",
            year
        );

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
                error!("❌ Failed to fetch yearly failed transactions for {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseYearStatusFailed> = results
            .into_iter()
            .map(TransactionResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed transaction records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed transactions for year {year}"),
            data: response_data,
        })
    }
}
