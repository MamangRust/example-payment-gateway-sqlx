use reqwest::{Client, Error};
use tracing::info;

use crate::{
    domain::{
        requests::role::{
            list::FindAllRole,
            trashed::{delete::DeletePermanentRole, restore::RestoreRoleTrashed},
        },
        response::role::{
            ApiResponsePaginationRoleDeleteAt, ApiResponseRole, ApiResponseRoleAll,
            ApiResponseRoleDelete,
        },
    },
    helpers::api::create_client,
};

pub struct RoleTrashedService {
    base_url: String,
    client: Client,
}

impl RoleTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_roles_trashed(
        &self,
        access_token: &str,
        req: FindAllRole,
    ) -> Result<ApiResponsePaginationRoleDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/roles/trashed", self.base_url))
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
            .json::<ApiResponsePaginationRoleDeleteAt>()
            .await?;

        info!("response = {:?}", response);

        Ok(response)
    }

    pub async fn restore_role_trashed(
        &self,
        access_token: &str,
        req: RestoreRoleTrashed,
    ) -> Result<ApiResponseRole, Error> {
        let response = self
            .client
            .post(format!("{}/roles/restore/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseRole>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_role(
        &self,
        access_token: &str,
        req: DeletePermanentRole,
    ) -> Result<ApiResponseRoleDelete, Error> {
        let response = self
            .client
            .post(format!("{}/roles/delete/{}", self.base_url, req.id))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseRoleDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_role_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseRoleAll, Error> {
        let response = self
            .client
            .post(format!("{}/roles/restore-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseRoleAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_role(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseRoleAll, Error> {
        let response = self
            .client
            .post(format!("{}/roles/permanent-all", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseRoleAll>()
            .await?;

        Ok(response)
    }
}
