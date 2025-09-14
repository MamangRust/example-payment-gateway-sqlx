use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::transaction::{
            list::FindAllTransaction,
            trashed::{delete::DeletePermanentTransaction, restore::RestoreTransactionTrashed},
        },
        response::transaction::{
            ApiResponsePaginationTransactionDeleteAt, ApiResponseTransaction,
            ApiResponseTransactionAll, ApiResponseTransactionDelete,
        },
    },
    helpers::api::create_client,
};

pub struct TransactionTrashedService {
    base_url: String,
    client: Client,
}

impl TransactionTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_transactions_trashed(
        &self,
        access_token: &str,
        req: FindAllTransaction,
    ) -> Result<ApiResponsePaginationTransactionDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/transactions/trashed", self.base_url))
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
            .json::<ApiResponsePaginationTransactionDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_transaction_trashed(
        &self,
        access_token: &str,
        req: RestoreTransactionTrashed,
    ) -> Result<ApiResponseTransaction, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/restore/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransaction>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_transaction(
        &self,
        access_token: &str,
        req: DeletePermanentTransaction,
    ) -> Result<ApiResponseTransactionDelete, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/delete/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransactionDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_trasnaction_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTransactionAll, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/restore-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransactionAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_transaction(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseTransactionAll, Error> {
        let response = self
            .client
            .post(format!("{}/transactions/delete-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseTransactionAll>()
            .await?;

        Ok(response)
    }
}
