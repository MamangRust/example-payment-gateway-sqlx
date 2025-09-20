use reqwest::{Client, Error};
use tracing::info;

use crate::{
    domain::{
        requests::user::{
            list::FindAllUser,
            trashed::{delete::DeletePermanentUser, restore::RestoreUserTrashed},
        },
        response::user::{
            ApiResponsePaginationUserDeleteAt, ApiResponseUser, ApiResponseUserAll,
            ApiResponseUserDelete,
        },
    },
    helpers::api::create_client,
};

pub struct UserTrashedService {
    base_url: String,
    client: Client,
}

impl UserTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_users_trashed(
        &self,
        access_token: &str,
        req: FindAllUser,
    ) -> Result<ApiResponsePaginationUserDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/users/trashed", self.base_url))
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
            .json::<ApiResponsePaginationUserDeleteAt>()
            .await?;

        info!("response = {:?}", response);

        Ok(response)
    }

    pub async fn restore_user_trashed(
        &self,
        access_token: &str,
        req: RestoreUserTrashed,
    ) -> Result<ApiResponseUser, Error> {
        let response = self
            .client
            .post(format!("{}/users/restore/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseUser>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_user(
        &self,
        access_token: &str,
        req: DeletePermanentUser,
    ) -> Result<ApiResponseUserDelete, Error> {
        let response = self
            .client
            .post(format!("{}/users/delete/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseUserDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_user_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseUserAll, Error> {
        let response = self
            .client
            .post(format!("{}/users/restore-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseUserAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_user(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseUserAll, Error> {
        let response = self
            .client
            .post(format!("{}/users/delete-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseUserAll>()
            .await?;

        Ok(response)
    }
}
