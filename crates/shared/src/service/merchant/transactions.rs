use crate::{
    abstract_trait::merchant::{
        repository::transactions::DynMerchantTransactionRepository,
        service::transactions::MerchantTransactionServiceTrait,
    },
    domain::{
        requests::merchant::{
            FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById,
        },
        responses::{ApiResponsePagination, MerchantTransactionResponse, Pagination},
    },
    errors::{RepositoryError, ServiceError},
    utils::mask_api_key,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantTransactionService {
    transaction: DynMerchantTransactionRepository,
}

impl MerchantTransactionService {
    pub async fn new(transaction: DynMerchantTransactionRepository) -> Self {
        Self { transaction }
    }
}

#[async_trait]
impl MerchantTransactionServiceTrait for MerchantTransactionService {
    async fn find_all(
        &self,
        req: &FindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all merchant transactions | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self
            .transaction
            .find_all_transactiions(req)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch all merchant transactions: {e:?}");
                ServiceError::Custom(e.to_string())
            })?;

        info!("✅ Found {} merchant transactions", transactions.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Merchant transactions retrieved successfully".to_string(),
            data: response_data,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_all_by_api_key(
        &self,
        req: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let masked_key = mask_api_key(&req.api_key);

        info!(
            "🔑 Fetching transactions by API key | Key: {masked_key}, Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self
            .transaction
            .find_all_transactiions_by_api_key(req)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch transactions for API key {masked_key}: {e:?}",);
                match e {
                    RepositoryError::NotFound => {
                        ServiceError::NotFound("No transactions found for this API key".to_string())
                    }
                    _ => ServiceError::InternalServerError(e.to_string()),
                }
            })?;

        info!(
            "✅ Retrieved {} transactions for API key: {masked_key}",
            transactions.len(),
        );

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by API key retrieved successfully".to_string(),
            data: response_data,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_all_by_id(
        &self,
        req: &FindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🆔 Fetching transactions by merchant ID: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            req.merchant_id,
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self
            .transaction
            .find_all_transactiions_by_id(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch transactions for merchant ID {}: {e:?}",
                    req.merchant_id,
                );
                match e {
                    RepositoryError::NotFound => ServiceError::NotFound(
                        "Merchant not found or has no transactions".to_string(),
                    ),
                    _ => ServiceError::InternalServerError(e.to_string()),
                }
            })?;

        info!(
            "✅ Found {} transactions for merchant ID {}",
            transactions.len(),
            req.merchant_id
        );

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let response_data: Vec<MerchantTransactionResponse> = transactions
            .into_iter()
            .map(MerchantTransactionResponse::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by merchant ID retrieved successfully".to_string(),
            data: response_data,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
}
