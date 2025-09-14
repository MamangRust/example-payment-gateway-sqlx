use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::card::{
            cardnumber::FindByCardNumber, create::CreateCard, findid::FindByIdCard,
            list::FindAllCard, update::UpdateCard, user::FindByUser,
        },
        response::card::{
            ApiResponseCard, ApiResponseDashboardCard, ApiResponseDashboardCardNumber,
            ApiResponseMonthlyAmount, ApiResponseMonthlyBalance, ApiResponsePaginationCard,
            ApiResponsePaginationCardDeleteAt, ApiResponseYearlyAmount, ApiResponseYearlyBalance,
        },
    },
    helpers::api::create_client,
};

pub struct CardService {
    base_url: String,
    client: Client,
}

impl CardService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_dashboard(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseDashboardCard, Error> {
        let response = self
            .client
            .get(format!("{}/cards/dashboard", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseDashboardCard>()
            .await?;

        Ok(response)
    }

    pub async fn find_dashboard_by_card_number(
        &self,
        access_token: &str,
        card_number: &str,
    ) -> Result<ApiResponseDashboardCardNumber, Error> {
        let url = format!("{}/cards/dashboard/{}", self.base_url, card_number);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseDashboardCardNumber>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_balance(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyBalance, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/balance/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyBalance>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_balance(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyBalance, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/balance/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyBalance>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_topup_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/topup/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn findyear_topup_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/topup/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_withdraw_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/withdraw/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn findyear_withdraw_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/withdraw/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_sender_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/monthly/sender",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_sender_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/yearly/sender",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_receiver_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/monthly/receiver",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_receiver_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/yearly/receiver",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/transaction/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_amount(
        &self,
        access_token: &str,
        year: i32,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/stats/transaction/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_balance_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyBalance, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/balance/monthly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyBalance>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_balance_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyBalance, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/balance/yearly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyBalance>()
            .await?;
        Ok(response)
    }

    pub async fn find_month_topup_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/topup/monthly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn findyear_topup_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/topup/yearly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_withdraw_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/withdraw/monthly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn findyear_withdraw_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/cards/withdraw/yearly/by-card", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_sender_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/monthly/by-card/sender",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_sender_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/yearly/by-card/sender",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transfer_receiver_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/monthly/by-card/receiver",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transfer_receiver_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transfer/yearly/by-card/receiver",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_transaction_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transaction/monthly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_transaction_amount_by_card(
        &self,
        access_token: &str,
        year: i32,
        card_number: &str,
    ) -> Result<ApiResponseYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/stats/transaction/yearly/by-card",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .query(&[("card_number", card_number)])
            .send()
            .await?
            .json::<ApiResponseYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_card(
        &self,
        access_token: &str,
        req: FindAllCard,
    ) -> Result<ApiResponsePaginationCard, Error> {
        let response = self
            .client
            .get(format!("{}/cards", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("page", req.page), ("page_size", req.page_size)])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationCard>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id_card(
        &self,
        access_token: &str,
        req: FindByIdCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .get(format!("{}/cards/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_user(
        &self,
        access_token: &str,
        req: FindByUser,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .get(format!("{}/cards/user/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_card_number(
        &self,
        access_token: &str,
        req: FindByCardNumber,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .get(format!(
                "{}/cards/cards_number/{}",
                self.base_url, req.card_number
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn find_active_card(
        &self,
        access_token: &str,
        req: FindAllCard,
    ) -> Result<ApiResponsePaginationCardDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/cards/active", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("page", req.page), ("page_size", req.page_size)])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationCardDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_card(
        &self,
        access_token: &str,
        req: CreateCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .post(format!("{}/cards/create", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "user_id": req.user_id,
                "card_type": req.card_type,
                "expire_date": req.expire_date,
                "cvv": req.cvv,
                "card_provider": req.card_provider,
            }))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn update_card(
        &self,
        access_token: &str,
        req: UpdateCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .post(format!("{}/cards/update/{}", self.base_url, req.card_id))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "user_id": req.user_id,
                "card_type": req.card_type,
                "expire_date": req.expire_date,
                "cvv": req.cvv,
                "card_provider": req.card_provider,
            }))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_card(
        &self,
        access_token: &str,
        req: FindByIdCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .get(format!("{}/cards/trash/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }
}
