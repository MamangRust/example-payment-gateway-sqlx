use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::merchant::{
            list::FindAllMerchant,
            trashed::{delete::DeletePermanentMerchant, restore::RestoreMerchantTrashed},
        },
        response::merchant::{
            ApiResponseMerchant, ApiResponseMerchantAll, ApiResponseMerchantDelete,
            ApiResponsePaginationMerchantDeleteAt,
        },
    },
    helpers::api::create_client,
};

pub struct MerchantTrashedService {
    base_url: String,
    client: Client,
}

impl MerchantTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_merchants_trashed(
        &self,
        access_token: &str,
        req: FindAllMerchant,
    ) -> Result<ApiResponsePaginationMerchantDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/merchants/trashed", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
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
            .json::<ApiResponsePaginationMerchantDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_merchant_trashed(
        &self,
        access_token: &str,
        req: RestoreMerchantTrashed,
    ) -> Result<ApiResponseMerchant, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/restore/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchant>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_merchant(
        &self,
        access_token: &str,
        req: DeletePermanentMerchant,
    ) -> Result<ApiResponseMerchantDelete, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/delete/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchantDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_merchant_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseMerchantAll, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/restore-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchantAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_merchant(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseMerchantAll, Error> {
        let response = self
            .client
            .post(format!("{}/merchants/delete-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseMerchantAll>()
            .await?;

        Ok(response)
    }
}
