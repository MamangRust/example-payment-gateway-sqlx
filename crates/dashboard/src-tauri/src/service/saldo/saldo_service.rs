use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::saldo::{
            create::CreateSaldo, findbyid::FindByIdSaldo, list::FindAllSaldo, update::UpdateSaldo,
        },
        response::saldo::{
            ApiResponseMonthSaldoBalances, ApiResponseMonthTotalSaldo, ApiResponsePaginationSaldo,
            ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseYearSaldoBalances,
            ApiResponseYearTotalSaldo,
        },
    },
    helpers::api::create_client,
};

pub struct SaldoService {
    base_url: String,
    client: Client,
}

impl SaldoService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_month_total_balance(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseMonthTotalSaldo, Error> {
        let response = self
            .client
            .get(format!(
                "{}/saldos/stats/total-balance/monthly",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseMonthTotalSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_total_balance(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseYearTotalSaldo, Error> {
        let response = self
            .client
            .get(format!(
                "{}/saldos/stats/total-balance/yearly",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearTotalSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_balance(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMonthSaldoBalances, Error> {
        let response = self
            .client
            .get(format!("{}/saldos/stats/balance/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthSaldoBalances>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_balance(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseYearSaldoBalances, Error> {
        let response = self
            .client
            .get(format!("{}/saldos/stats/balance/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearSaldoBalances>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_saldos(
        &self,
        access_token: &str,
        req: FindAllSaldo,
    ) -> Result<ApiResponsePaginationSaldo, Error> {
        let response = self
            .client
            .get(format!("{}/saldos", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationSaldo>()
            .await?;

        println!("{:#?}", response);

        Ok(response)
    }

    pub async fn find_by_id(
        &self,
        access_token: &str,
        req: FindByIdSaldo,
    ) -> Result<ApiResponseSaldo, Error> {
        let response = self
            .client
            .get(format!("{}/saldos/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active(
        &self,
        access_token: &str,
        req: FindAllSaldo,
    ) -> Result<ApiResponsePaginationSaldoDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/saldos/active", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationSaldoDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_saldo(
        &self,
        access_token: &str,
        req: CreateSaldo,
    ) -> Result<ApiResponseSaldo, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/create", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "card_number": req.card_number,
                "total_balance": req.total_balance,
            }))
            .send()
            .await?
            .json::<ApiResponseSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn update_saldo(
        &self,
        access_token: &str,
        req: UpdateSaldo,
    ) -> Result<ApiResponseSaldo, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/update/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                 "card_number": req.card_number,
                "total_balance": req.total_balance,
            }))
            .send()
            .await?
            .json::<ApiResponseSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_saldo(
        &self,
        access_token: &str,
        req: FindByIdSaldo,
    ) -> Result<ApiResponseSaldo, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/trash/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldo>()
            .await?;

        Ok(response)
    }
}
