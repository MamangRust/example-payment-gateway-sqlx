use reqwest::{Client, Error};
use serde_json::json;
use tracing::info;

use crate::{
    domain::{
        requests::withdraw::{
            create::CreateWithdraw, findbycardnumber::FindByCardNumberWithdraw,
            findbyid::FindByIdWithdraw, list::FindAllWithdraw, trashedwithdraw::TrashedWithdraw,
            update::UpdateWithdraw,
        },
        response::withdraw::{
            ApiResponsePaginationWithdraw, ApiResponsePaginationWithdrawDeleteAt,
            ApiResponseWithdraw, ApiResponseWithdrawMonthAmount,
            ApiResponseWithdrawMonthStatusFailed, ApiResponseWithdrawMonthStatusSuccess,
            ApiResponseWithdrawYearAmount, ApiResponseWithdrawYearStatusFailed,
            ApiResponseWithdrawYearStatusSuccess,
        },
    },
    helpers::api::create_client,
};

pub struct WithdrawService {
    base_url: String,
    client: Client,
}

impl WithdrawService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_month_status_success(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseWithdrawMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/success/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseWithdrawYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/success/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseWithdrawMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/failed/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseWithdrawYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/failed/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/success/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/success/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/failed/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/status/failed/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_withdraw_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseWithdrawMonthAmount, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws/stats/amount/monthly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_withdraw_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseWithdrawYearAmount, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws/stats/amount/yearly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_withdraw_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/amount/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_withdraw_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseWithdrawYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/stats/amount/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseWithdrawYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_withdraws(
        &self,
        access_token: &str,
        req: FindAllWithdraw,
    ) -> Result<ApiResponsePaginationWithdraw, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws", self.base_url))
            .bearer_auth(access_token)
            .query(&[("page", req.page), ("page_size", req.page_size)])
            .query(&[(
                "search",
                if req.search.is_empty() {
                    ""
                } else {
                    &req.search
                },
            )])
            .send()
            .await?
            .json::<ApiResponsePaginationWithdraw>()
            .await?;

        info!("response = {:?}", response);

        Ok(response)
    }

    pub async fn find_by_id_withdraw(
        &self,
        access_token: &str,
        req: FindByIdWithdraw,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_card_number_withdraw(
        &self,
        access_token: &str,
        req: FindByCardNumberWithdraw,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .get(format!(
                "{}/withdraws/by-card/{}",
                self.base_url, req.card_number
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active_withdraw(
        &self,
        access_token: &str,
        req: FindAllWithdraw,
    ) -> Result<ApiResponsePaginationWithdrawDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws/active", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationWithdrawDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_withdraw(
        &self,
        access_token: &str,
        req: CreateWithdraw,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/create", self.base_url))
            .bearer_auth(access_token)
            .json(&json!({
               "card_number": req.card_number,
                "withdraw_amount": req.withdraw_amount,
                "withdraw_time": req.withdraw_time
            }))
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }

    pub async fn update_withdraw(
        &self,
        access_token: &str,
        req: UpdateWithdraw,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/update/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .json(&json!({
                "card_number": req.card_number,
                "withdraw_amount": req.withdraw_amount,
                "withdraw_time": req.withdraw_time
            }))
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_withdraw(
        &self,
        access_token: &str,
        req: TrashedWithdraw,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/trash/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }
}
