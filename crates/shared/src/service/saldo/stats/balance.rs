use crate::{
    abstract_trait::saldo::{
        repository::stats::balance::DynSaldoBalanceRepository,
        service::stats::balance::SaldoBalanceServiceTrait,
    },
    domain::responses::{ApiResponse, SaldoMonthBalanceResponse, SaldoYearBalanceResponse},
    errors::ServiceError,
};
use async_trait::async_trait;
use tracing::{error, info};

pub struct SaldoBalanceService {
    balance: DynSaldoBalanceRepository,
}

impl SaldoBalanceService {
    pub async fn new(balance: DynSaldoBalanceRepository) -> Self {
        Self { balance }
    }
}

#[async_trait]
impl SaldoBalanceServiceTrait for SaldoBalanceService {
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoMonthBalanceResponse>>, ServiceError> {
        info!("📅💰 Fetching monthly balance for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let balances = self.balance.get_month_balance(year).await.map_err(|e| {
            error!("❌ Failed to retrieve monthly balance for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<SaldoMonthBalanceResponse> = balances
            .into_iter()
            .map(SaldoMonthBalanceResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly balance records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly balance for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearBalanceResponse>>, ServiceError> {
        info!("📆💰 Fetching yearly balance for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let balances = self.balance.get_year_balance(year).await.map_err(|e| {
            error!("❌ Failed to retrieve yearly balance for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<SaldoYearBalanceResponse> = balances
            .into_iter()
            .map(SaldoYearBalanceResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly balance records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly balance for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
