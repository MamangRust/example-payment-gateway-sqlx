use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::transaction::{
            create::CreateTransaction, findbycardnumber::FindByCardNumberTransaction,
            findbyid::FindByIdTransaction, findbyidmerchant::FindByMerchantTransaction,
            list::FindAllTransaction, trashedtransaction::TrashedTransaction,
            update::UpdateTransaction,
        },
        response::transaction::{
            ApiResponsePaginationTransaction, ApiResponsePaginationTransactionDeleteAt,
            ApiResponseTransaction, ApiResponseTransactionMonthAmount,
            ApiResponseTransactionMonthMethod, ApiResponseTransactionMonthStatusFailed,
            ApiResponseTransactionMonthStatusSuccess, ApiResponseTransactionYearAmount,
            ApiResponseTransactionYearMethod, ApiResponseTransactionYearStatusFailed,
            ApiResponseTransactionYearStatusSuccess, ApiResponseTransactions,
        },
    },
    helpers::api::create_client,
};

pub struct TransactionService {
    base_url: String,
    client: Client,
}

impl TransactionService {
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
    ) -> Result<ApiResponseTransactionMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/success/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/success/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseTransactionMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/failed/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/failed/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionMonthStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/success/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_success_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionYearStatusSuccess, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/success/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearStatusSuccess>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionMonthStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/failed/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year), ("month", month)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_status_failed_by_card_number(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionYearStatusFailed, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/status/failed/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearStatusFailed>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionMonthMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/method/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionYearMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/method/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/amount/monthly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseTransactionYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/amount/yearly",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_method_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionMonthMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/method/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_method_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionYearMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/method/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionMonthAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/amount/monthly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionMonthAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_amount_by_card(
        &self,
        access_token: &str,
        year: u32,
        card_number: &str,
    ) -> Result<ApiResponseTransactionYearAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/stats/amount/yearly/by-card",
                self.base_url
            ))
            .bearer_auth(access_token)
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseTransactionYearAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_transactions(
        &self,
        access_token: &str,
        req: FindAllTransaction,
    ) -> Result<ApiResponsePaginationTransaction, Error> {
        let response = self
            .client
            .get(format!("{}/transactions", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id_transaction(
        &self,
        access_token: &str,
        req: FindByIdTransaction,
    ) -> Result<ApiResponseTransaction, Error> {
        let response = self
            .client
            .get(format!("{}/transactions/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_card_number_transaction(
        &self,
        access_token: &str,
        req: FindByCardNumberTransaction,
    ) -> Result<ApiResponseTransactions, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/by-card/{}",
                self.base_url, req.card_number
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransactions>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_merchant_transaction(
        &self,
        access_token: &str,
        req: FindByMerchantTransaction,
    ) -> Result<ApiResponseTransactions, Error> {
        let response = self
            .client
            .get(format!(
                "{}/transactions/merchant/{}",
                self.base_url, req.id
            ))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransactions>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active_transaction(
        &self,
        access_token: &str,
        req: FindAllTransaction,
    ) -> Result<ApiResponsePaginationTransactionDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/transactions/active", self.base_url))
            .bearer_auth(access_token)
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationTransactionDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_transaction(
        &self,
        access_token: &str,
        req: CreateTransaction,
    ) -> Result<ApiResponseTransaction, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/create", self.base_url))
            .bearer_auth(access_token)
            .header("X-API-Key", req.api_key)
            .json(&json!({
                "card_number": req.card_number,
                "amount": req.amount,
                "merchant_id": req.merchant_id,
                "payment_method": req.payment_method,
                "transaction_time": req.transaction_time,
            }))
            .send()
            .await?
            .json::<ApiResponseTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn update_transaction(
        &self,
        access_token: &str,
        req: UpdateTransaction,
    ) -> Result<ApiResponseTransaction, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/update/{}", self.base_url, req.id,))
            .bearer_auth(access_token)
            .header("X-API-Key", req.api_key)
            .json(&json!({
                "transaction_id": req.id,
                "card_number": req.card_number,
                "amount": req.amount,
                "merchant_id": req.merchant_id,
                "payment_method": req.payment_method,
                "transaction_time": req.transaction_time,
            }))
            .send()
            .await?
            .json::<ApiResponseTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_transaction(
        &self,
        access_token: &str,
        req: TrashedTransaction,
    ) -> Result<ApiResponseTransaction, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/trash/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTransaction>()
            .await?;

        Ok(response)
    }
}
