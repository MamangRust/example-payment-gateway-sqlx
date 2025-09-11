use crate::{
    abstract_trait::saldo::{
        repository::stats::total::DynSaldoTotalBalanceRepository,
        service::stats::total::SaldoTotalBalanceServiceTrait,
    },
    domain::{
        requests::saldo::MonthTotalSaldoBalance,
        responses::{ApiResponse, SaldoMonthTotalBalanceResponse, SaldoYearTotalBalanceResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct SaldoTotalBalanceService {
    total_balance: DynSaldoTotalBalanceRepository,
}

impl SaldoTotalBalanceService {
    pub async fn new(total_balance: DynSaldoTotalBalanceRepository) -> Self {
        Self { total_balance }
    }
}

#[async_trait]
impl SaldoTotalBalanceServiceTrait for SaldoTotalBalanceService {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, ServiceError> {
        info!("📅💵 Fetching monthly total balance for year: {}", req.year);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let balances = self
            .total_balance
            .get_month_total_balance(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to retrieve monthly total balance for year {}: {e:?}",
                    req.year
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<SaldoMonthTotalBalanceResponse> = balances
            .into_iter()
            .map(SaldoMonthTotalBalanceResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly total balance records",
            response_data.len()
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly total balance for year {} retrieved successfully",
                req.year
            ),
            data: response_data,
        })
    }

    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, ServiceError> {
        info!("📆💵 Fetching yearly total balance for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let balances = self
            .total_balance
            .get_year_total_balance(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to retrieve yearly total balance for year {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<SaldoYearTotalBalanceResponse> = balances
            .into_iter()
            .map(SaldoYearTotalBalanceResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly total balance records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly total balance for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
