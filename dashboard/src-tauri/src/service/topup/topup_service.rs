use reqwest::{Client, Error};
use serde_json::json;

use crate::domain::{
    requests::topup::{
        create::CreateTopup, findbycardnumber::FindByCardNumberTopup, findbyid::FindByIdTopup,
        list::FindAllTopup, trashedtopup::TrashedTopup, update::UpdateTopup,
    },
    response::topup::{
        ApiResponsePaginationTopup, ApiResponsePaginationTopupDeleteAt, ApiResponseTopup,
        ApiResponseTopupMonthAmount, ApiResponseTopupMonthMethod,
        ApiResponseTopupMonthStatusFailed, ApiResponseTopupMonthStatusSuccess,
        ApiResponseTopupYearAmount, ApiResponseTopupYearMethod, ApiResponseTopupYearStatusFailed,
        ApiResponseTopupYearStatusSuccess,
    },
};

pub struct TopupService {
    base_url: String,
    client: Client,
}

impl TopupService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn find_month_status_success(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseTopupMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/success/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/success/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseTopupMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/failed/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/failed/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_success_by_card(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/success/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/success/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed_by_card(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/failed/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/status/failed/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_topup_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupMonthMethod, Error> {
        let response = self
            .client
            .get(format!("{}/topups/stats/method/monthly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_topup_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupYearMethod, Error> {
        let response = self
            .client
            .get(format!("{}/topups/stats/method/yearly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupYearMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_topup_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupMonthAmount, Error> {
        let response = self
            .client
            .get(format!("{}/topups/stats/amount/monthly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_topup_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTopupYearAmount, Error> {
        let response = self
            .client
            .get(format!("{}/topups/stats/amount/yearly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTopupYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_topup_method_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupMonthMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/method/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_topup_method_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupYearMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/method/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupYearMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_topup_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/amount/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_topup_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTopupYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/stats/amount/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTopupYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_topups(
        &self,
        access_token: &str,
        req: FindAllTopup,
    ) -> Result<ApiResponsePaginationTopup, Error> {
        let response = self
            .client
            .get(format!("{}/topups", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTopup>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id_topup(
        &self,
        access_token: &str,
        req: FindByIdTopup,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .get(format!("{}/topups/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active_topup(
        &self,
        access_token: &str,
        req: FindAllTopup,
    ) -> Result<ApiResponsePaginationTopupDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/topups/active", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTopupDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_card_number_topup(
        &self,
        access_token: &str,
        req: FindByCardNumberTopup,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .get(format!(
                "{}/topups/by-card/{}",
                self.base_url, req.card_number
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }

    pub async fn create_topup(
        &self,
        access_token: &str,
        req: CreateTopup,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .post(format!("{}/topups/create", self.base_url))
            .bearer_auth(access_token)
            .json(&json!({
                "card_number": req.card_number,
                "topup_amount": req.topup_amount,
                "topup_method": req.topup_method,
            }))
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }

    pub async fn update_topup(
        &self,
        access_token: &str,
        req: UpdateTopup,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .post(format!("{}/topups/update/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .json(&json!({
                "card_number": req.card_number,
                "topup_amount": req.topup_amount,
                "topup_method": req.topup_method,
            }))
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_topup(
        &self,
        access_token: &str,
        req: TrashedTopup,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .post(format!("{}/topups/trash/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }
}
