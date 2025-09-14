use reqwest::{Client, Error};

use crate::{
    domain::{
        requests::saldo::{
            list::FindAllSaldo,
            trashed::{delete::DeletePermanentSaldo, restore::RestoreSaldoTrashed},
        },
        response::saldo::{
            ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseSaldoAll,
            ApiResponseSaldoDelete,
        },
    },
    helpers::api::create_client,
};

pub struct SaldoTrashedService {
    base_url: String,
    client: Client,
}

impl SaldoTrashedService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn find_all_saldos_trashed(
        &self,
        access_token: &str,
        req: FindAllSaldo,
    ) -> Result<ApiResponsePaginationSaldoDeleteAt, Error> {
        let response = self
            .client
            .get(format!("{}/saldos/trashed", self.base_url))
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
            .json::<ApiResponsePaginationSaldoDeleteAt>()
            .await?;

        Ok(response)
    }

    pub async fn restore_saldo_trashed(
        &self,
        access_token: &str,
        req: RestoreSaldoTrashed,
    ) -> Result<ApiResponseSaldo, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/restore/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldo>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_saldo(
        &self,
        access_token: &str,
        req: DeletePermanentSaldo,
    ) -> Result<ApiResponseSaldoDelete, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/delete/{}", self.base_url, req.id))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldoDelete>()
            .await?;

        Ok(response)
    }

    pub async fn restore_saldo_all_trashed(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseSaldoAll, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/restore-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldoAll>()
            .await?;

        Ok(response)
    }

    pub async fn delete_permanent_all_saldo(
        &self,
        access_token: &str,
    ) -> Result<ApiResponseSaldoAll, Error> {
        let response = self
            .client
            .post(format!("{}/saldos/delete-all", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<ApiResponseSaldoAll>()
            .await?;

        Ok(response)
    }
}
