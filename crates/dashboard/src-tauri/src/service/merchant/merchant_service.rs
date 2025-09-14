use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::merchant::{
            create::CreateMerchant,
            findbyapikey::FindByApiKeyMerchant,
            findbyid::FindByIdMerchant,
            findmerchantuser::FindMerchantUser,
            list::{FindAllMerchant, FindAllMerchantTransaction, FindAllMerchantTransactionApiKey},
            trashedmerchant::FindTrashedMerchant,
            update::UpdateMerchant,
        },
        response::merchant::{
            ApiResponseMerchant, ApiResponseMerchantMonthlyAmount,
            ApiResponseMerchantMonthlyPaymentMethod, ApiResponseMerchantMonthlyTotalAmount,
            ApiResponseMerchantYearlyAmount, ApiResponseMerchantYearlyPaymentMethod,
            ApiResponseMerchantYearlyTotalAmount, ApiResponsePaginationMerchant,
            ApiResponsePaginationMerchantDeleteAt, ApiResponsePaginationMerchantTransaction,
            ApiResponsesMerchant,
        },
    },
    helpers::api::create_client,
};

pub struct MerchantService {
    base_url: String,
    client: Client,
}

impl MerchantService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_month_payment_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMerchantMonthlyPaymentMethod, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/stats/method/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyPaymentMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_payment_method(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMerchantYearlyPaymentMethod, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/stats/method/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyPaymentMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMerchantMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/stats/amount/monthly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMerchantYearlyAmount, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/stats/amount/yearly", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_total_amount(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
    ) -> Result<ApiResponseMerchantMonthlyTotalAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/total-amount/monthly",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("month", month)])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyTotalAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_total_amount(
        &self,
        access_token: &str,
        year: u32,
    ) -> Result<ApiResponseMerchantYearlyTotalAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/total-amount/yearly",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyTotalAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_payment_method_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantMonthlyPaymentMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/method/monthly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("merchant_id", merchant_id)])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyPaymentMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_payment_method_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantYearlyPaymentMethod, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/method/yearly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("merchant_id", merchant_id)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyPaymentMethod>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_amount_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantMonthlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/amount/monthly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("merchant_id", merchant_id)])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_amount_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantYearlyAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/amount/yearly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("merchant_id", merchant_id)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_month_total_amount_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        month: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantMonthlyTotalAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/total-amount/monthly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("year", year),
                ("month", month),
                ("merchant_id", merchant_id),
            ])
            .send()
            .await?
            .json::<ApiResponseMerchantMonthlyTotalAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_year_total_amount_by_merchant(
        &self,
        access_token: &str,
        year: u32,
        merchant_id: u32,
    ) -> Result<ApiResponseMerchantYearlyTotalAmount, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/stats/total-amount/yearly/by-merchant",
                self.base_url
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[("year", year), ("merchant_id", merchant_id)])
            .send()
            .await?
            .json::<ApiResponseMerchantYearlyTotalAmount>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_merchants(
        &self,
        access_token: &str,
        req: FindAllMerchant,
    ) -> Result<ApiResponsePaginationMerchant, Error> {
        let response = self
            .client
            .get(format!("{}/merchants", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_transactions(
        &self,
        access_token: &str,
        req: FindAllMerchant,
    ) -> Result<ApiResponsePaginationMerchantTransaction, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/transactions", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationMerchantTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_transactions_by_merchant(
        &self,
        access_token: &str,
        req: FindAllMerchantTransaction,
    ) -> Result<ApiResponsePaginationMerchantTransaction, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/transactions/{}/",
                self.base_url, req.merchant_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationMerchantTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn find_all_transactions_by_api_key(
        &self,
        access_token: &str,
        req: FindAllMerchantTransactionApiKey,
    ) -> Result<ApiResponsePaginationMerchantTransaction, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/transactions/api-key/{}/",
                self.base_url, req.api_key
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationMerchantTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id(
        &self,
        access_token: &str,
        req: FindByIdMerchant,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_api_key(
        &self,
        access_token: &str,
        req: FindByApiKeyMerchant,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/api-key/{}",
                self.base_url, req.api_key
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_merchant_user(
        &self,
        access_token: &str,
        req: FindMerchantUser,
    ) -> Result<ApiResponsesMerchant, Error> {
        let response = self
            .client
            .get(format!(
                "{}/merchants/merchant-user/{}",
                self.base_url, req.user_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponsesMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active(
        &self,
        access_token: &str,
        req: FindAllMerchant,
    ) -> Result<ApiResponsePaginationMerchantDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/active", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationMerchantDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_merchant(
        &self,
        access_token: &str,
        req: CreateMerchant,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/create", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "name": req.name,
                "user_id": req.user_id,
            }))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn update_merchant(
        &self,
        access_token: &str,
        req: UpdateMerchant,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .post(format!(
                "{}/merchants/update/{}",
                self.base_url, req.merchant_id
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "merchant_id": req.merchant_id,
                "name": req.name,
                "user_id": req.user_id,
                "status": req.status,
            }))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_merchant(
        &self,
        access_token: &str,
        req: FindTrashedMerchant,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/trash/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }
}
