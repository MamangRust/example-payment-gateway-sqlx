use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::withdraw::{
            list::FindAllWithdraw,
            trashed::{delete::DeletePermanentWithdraw, restore::RestoreWithdrawTrashed},
        },
        response::withdraw::{
            ApiResponsePaginationWithdrawDeleteAt, ApiResponseWithdraw, ApiResponseWithdrawAll,
            ApiResponseWithdrawDelete,
        },
    },
    helpers::api::create_client,
};

pub struct WithdrawTrashedService {
    base_url: String,
    client: Client,
}

impl WithdrawTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_withdraws_trashed(
        &self,
        access_token: &str,
        req: FindAllWithdraw,
    ) -> Result<ApiResponsePaginationWithdrawDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/withdraws/trashed", self.base_url))
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
            .json::<ApiResponsePaginationWithdrawDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_withdraw_trashed(
        &self,
        access_token: &str,
        req: RestoreWithdrawTrashed,
    ) -> Result<ApiResponseWithdraw, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/restore/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdraw>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_withdraw(
        &self,
        access_token: &str,
        req: DeletePermanentWithdraw,
    ) -> Result<ApiResponseWithdrawDelete, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/delete/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdrawDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_withdraw_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseWithdrawAll, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/restore-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdrawAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_withdraw(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseWithdrawAll, Error> {
        let response = self
            .client
            .post(format!("{}/withdraws/delete-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseWithdrawAll>()
            .await?;

        Ok(response)
    }
}
