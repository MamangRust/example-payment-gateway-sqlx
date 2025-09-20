use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::topup::{
            list::FindAllTopup,
            trashed::{delete::DeletePermanentTopup, restore::RestoreTopupTrashed},
        },
        response::topup::{
            ApiResponsePaginationTopupDeleteAt, ApiResponseTopup, ApiResponseTopupAll,
            ApiResponseTopupDelete,
        },
    },
    helpers::api::create_client,
};

pub struct TopupTrashedService {
    base_url: String,
    client: Client,
}

impl TopupTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_topups_trashed(
        &self,
        access_token: &str,
        req: FindAllTopup,
    ) -> Result<ApiResponsePaginationTopupDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/topups/trashed", self.base_url))
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
            .json::<ApiResponsePaginationTopupDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_topup_trashed(
        &self,
        access_token: &str,
        req: RestoreTopupTrashed,
    ) -> Result<ApiResponseTopup, Error> {
        let response = self
            .client
            .post(format!("{}/topups/restore/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopup>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_topup(
        &self,
        access_token: &str,
        req: DeletePermanentTopup,
    ) -> Result<ApiResponseTopupDelete, Error> {
        let response = self
            .client
            .post(format!("{}/topups/delete/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopupDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_topup_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTopupAll, Error> {
        let response = self
            .client
            .post(format!("{}/topups/restore-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopupAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_topup(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTopupAll, Error> {
        let response = self
            .client
            .post(format!("{}/topups/delete-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseTopupAll>()
            .await?;

        Ok(response)
    }
}
