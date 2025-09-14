use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::role::{
            create::CreateRole, findbyid::FindByIdRole, list::FindAllRole,
            trashed::list::FindAllRoleTrashed, trashedrole::TrashedRole, update::UpdateRole,
        },
        response::role::{
            ApiResponsePaginationRole, ApiResponsePaginationRoleDeleteAt, ApiResponseRole,
        },
    },
    helpers::api::create_client,
};

pub struct RoleService {
    base_url: String,
    client: Client,
}

impl RoleService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_roles(
        &self,
        access_token: &str,
        req: FindAllRole,
    ) -> Result<ApiResponsePaginationRole, Error> {
        let response = self
            .client
            .get(format!("{}/role", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationRole>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id(
        &self,
        access_token: &str,
        req: FindByIdRole,
    ) -> Result<ApiResponseRole, Error> {
        let response = self
            .client
            .get(format!("{}/role/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseRole>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active(
        &self,
        access_token: &str,
        req: FindAllRoleTrashed,
    ) -> Result<ApiResponsePaginationRoleDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/role/active", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationRoleDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_role(
        &self,
        access_token: &str,
        req: CreateRole,
    ) -> Result<ApiResponseRole, Error> {
        let response = self
            .client
            .post(format!("{}/role/create", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "name": req.name,
            }))
            .send()
            .await?
            .json::<ApiResponseRole>()
            .await?;

        Ok(response)
    }

    pub async fn update_role(
        &self,
        access_token: &str,
        req: UpdateRole,
    ) -> Result<ApiResponseRole, Error> {
        let response = self
            .client
            .post(format!("{}/role/update/{}", self.base_url, req.role_id))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "role_id": req.role_id,
                "name": req.name,
            }))
            .send()
            .await?
            .json::<ApiResponseRole>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_role(
        &self,
        access_token: &str,
        req: TrashedRole,
    ) -> Result<ApiResponseRole, Error> {
        let response = self
            .client
            .post(format!("{}/role/trashed/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseRole>()
            .await?;

        Ok(response)
    }
}
