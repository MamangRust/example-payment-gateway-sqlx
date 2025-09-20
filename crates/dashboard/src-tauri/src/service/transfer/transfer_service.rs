use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::transfer::{
            create::CreateTransfer, findbyid::FindByIdTransfer, list::FindAllTransfer,
            transferfrom::TransferFrom, transferto::TransferTo, trashedtransfer::TrashedTransfer,
            update::UpdateTransfer,
        },
        response::transfer::{
            ApiResponsePaginationTransfer, ApiResponsePaginationTransferDeleteAt,
            ApiResponseTransfer, ApiResponseTransferMonthAmount,
            ApiResponseTransferMonthStatusFailed, ApiResponseTransferMonthStatusSuccess,
            ApiResponseTransferYearAmount, ApiResponseTransferYearStatusFailed,
            ApiResponseTransferYearStatusSuccess, ApiResponseTransfers,
        },
    },
    helpers::api::create_client,
};

pub struct TransferService {
    base_url: String,
    client: Client,
}

impl TransferService {
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
    ) -> Result<ApiResponseTransferMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/success/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransferYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/success/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransferYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseTransferMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/tranfers/stats/status/failed/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransferYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/failed/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransferYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/success/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/success/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/tranfers/stats/status/failed/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/status/failed/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransferMonthAmount, Error> {
        let response = self
            .client
            .get(format!("{}/transfers/stats/amount/monthly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransferYearAmount, Error> {
        let response = self
            .client
            .get(format!("{}/transfers/stats/amount/yearly", self.base_url))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransferYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_amount_by_sender(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/amount/monthly/sender",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_amount_by_sender(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/stats/amount/yearly/sender",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_amount_by_receiver(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/amount/monthly/receiver",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_amount_by_receiver(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransferYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/amount/yearly/receiver",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransferYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_transfers(
        &self,
        access_token: &str,
        req: FindAllTransfer,
    ) -> Result<ApiResponsePaginationTransfer, Error> {
        let response = self
            .client
            .get(format!("{}/transfers", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTransfer>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id_transfer(
        &self,
        access_token: &str,
        req: FindByIdTransfer,
    ) -> Result<ApiResponseTransfer, Error> {
        let response = self
            .client
            .get(format!("{}/transfers/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransfer>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_transfer_from(
        &self,
        access_token: &str,
        req: TransferFrom,
    ) -> Result<ApiResponseTransfers, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/from/{}",
                self.base_url, req.card_number
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransfers>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_transfer_to(
        &self,
        access_token: &str,
        req: TransferTo,
    ) -> Result<ApiResponseTransfers, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transfers/to/{}",
                self.base_url, req.card_number
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransfers>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active_transfer(
        &self,
        access_token: &str,
        req: FindAllTransfer,
    ) -> Result<ApiResponsePaginationTransferDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/transfers/active", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTransferDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_transfer(
        &self,
        access_token: &str,
        req: CreateTransfer,
    ) -> Result<ApiResponseTransfer, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/create", self.base_url))
            .bearer_auth(access_token)
            .json(&json!({
                "transfer_from": req.transfer_from,
                "transfer_to": req.transfer_to,
                "transfer_amount":req.transfer_amount,
            }))
            .send()
            .await?
            .json::<ApiResponseTransfer>()
            .await?;

        Ok(response)
    }

    pub async fn update_transfer(
        &self,
        access_token: &str,
        req: UpdateTransfer,
    ) -> Result<ApiResponseTransfer, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/update/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .json(&json!({
                "transfer_from": req.transfer_from,
                "transfer_to": req.transfer_to,
                "transfer_amount":req.transfer_amount,
            }))
            .send()
            .await?
            .json::<ApiResponseTransfer>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_transfer(
        &self,
        access_token: &str,
        req: TrashedTransfer,
    ) -> Result<ApiResponseTransfer, Error> {
        let response = self
            .client
            .post(format!("{}/tranfers/trash/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransfer>()
            .await?;

        Ok(response)
    }
}
