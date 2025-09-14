use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::card::{findid::FindByIdCard, list::FindAllCard},
        response::card::{ApiResponseCard, ApiResponseCardAll, ApiResponsePaginationCardDeleteAt},
    },
    helpers::api::create_client,
};

pub struct CardTrashedService {
    base_url: String,
    client: Client,
}

impl CardTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_cards_trashed(
        &self,
        access_token: &str,
        req: FindAllCard,
    ) -> Result<ApiResponsePaginationCardDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/card/trashed", self.base_url))
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
            .json::<ApiResponsePaginationCardDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_card_trashed(
        &self,
        access_token: &str,
        req: FindByIdCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .post(format!("{}/card/restore/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_card(
        &self,
        access_token: &str,
        req: FindByIdCard,
    ) -> Result<ApiResponseCard, Error> {
        let response = self
            .client
            .post(format!("{}/card/delete/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCard>()
            .await?;

        Ok(response)
    }

    pub async fn restore_all_cards_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseCardAll, Error> {
        let response = self
            .client
            .post(format!("{}/card/restore-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCardAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_cards(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseCardAll, Error> {
        let response = self
            .client
            .post(format!("{}/card/delete-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseCardAll>()
            .await?;

        Ok(response)
    }
}
