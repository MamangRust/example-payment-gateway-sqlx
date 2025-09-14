use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::transfer::{
            list::FindAllTransfer,
            trashed::{delete::DeletePermanentTransfer, restore::RestoreTransferTrashed},
        },
        response::transfer::{
            ApiResponsePaginationTransferDeleteAt, ApiResponseTransfer, ApiResponseTransferAll,
            ApiResponseTransferDelete,
        },
    },
    helpers::api::create_client,
};

pub struct TransferTrashedService {
    base_url: String,
    client: Client,
}

impl TransferTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_transfers_trashed(
        &self,
        access_token: &str,
        req: FindAllTransfer,
    ) -> Result<ApiResponsePaginationTransferDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/transfers/trashed", self.base_url))
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
            .json::<ApiResponsePaginationTransferDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_transfer_trashed(
        &self,
        access_token: &str,
        req: RestoreTransferTrashed,
    ) -> Result<ApiResponseTransfer, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/restore/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransfer>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_transfer(
        &self,
        access_token: &str,
        req: DeletePermanentTransfer,
    ) -> Result<ApiResponseTransferDelete, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/delete/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransferDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_transfer_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTransferAll, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/restore-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransferAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_transfer(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTransferAll, Error> {
        let response = self
            .client
            .post(format!("{}/transfers/permanent-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransferAll>()
            .await?;

        Ok(response)
    }
}
