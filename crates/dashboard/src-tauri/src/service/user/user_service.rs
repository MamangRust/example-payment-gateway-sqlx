use reqwest::{Client, Error};
use serde_json::json;

use crate::{
    domain::{
        requests::user::{
            create::CreateUser, findbyid::FindByIdUser, list::FindAllUser,
            trashed::list::FindAllUserTrashed, trasheduser::TrashedUser, update::UpdateUser,
        },
        response::user::{
            ApiResponsePaginationUser, ApiResponsePaginationUserDeleteAt, ApiResponseUser,
        },
    },
    helpers::api::create_client,
};

pub struct UserService {
    base_url: String,
    client: Client,
}

impl UserService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_users(
        &self,
        access_token: &str,
        req: FindAllUser,
    ) -> Result<ApiResponsePaginationUser, Error> {
        let response = self
            .client
            .get(format!("{}/users", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationUser>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_id(
        &self,
        access_token: &str,
        req: FindByIdUser,
    ) -> Result<ApiResponseUser, Error> {
        let response = self
            .client
            .get(format!("{}/users/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseUser>()
            .await?;

        Ok(response)
    }

    pub async fn find_by_active(
        &self,
        access_token: &str,
        req: FindAllUserTrashed,
    ) -> Result<ApiResponsePaginationUserDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/users/active", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("page", req.page.to_string()),
                ("page_size", req.page_size.to_string()),
            ])
            .query(&[("search", req.search)])
            .send()
            .await?
            .json::<ApiResponsePaginationUserDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn create_user(
        &self,
        access_token: &str,
        req: CreateUser,
    ) -> Result<ApiResponseUser, Error> {
        let response = self
            .client
            .post(format!("{}/users/create", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "firstname": req.firstname,
                "lastname": req.lastname,
                "email": req.email,
                "password": req.password,
                "confirm_password": req.confirm_password,
            }))
            .send()
            .await?
            .json::<ApiResponseUser>()
            .await?;

        Ok(response)
    }

    pub async fn update_user(
        &self,
        access_token: &str,
        req: UpdateUser,
    ) -> Result<ApiResponseUser, Error> {
        let response = self
            .client
            .post(format!("{}/users/update/{}", self.base_url, req.user_id))
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({
                "user_id": req.user_id,
                "firstname": req.firstname,
                "lastname": req.lastname,
                "email": req.email,
                "password": req.password,
                "confirm_password": req.confirm_password,
            }))
            .send()
            .await?
            .json::<ApiResponseUser>()
            .await?;

        Ok(response)
    }

    pub async fn trashed_user(
        &self,
        access_token: &str,
        req: TrashedUser,
    ) -> Result<ApiResponseUser, Error> {
        let response = self
            .client
            .post(format!("{}/users/trash/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseUser>()
            .await?;

        Ok(response)
    }
}
