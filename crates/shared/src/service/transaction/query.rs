use crate::{
    abstract_trait::transaction::{
        repository::query::DynTransactionQueryRepository,
        service::query::TransactionQueryServiceTrait,
    },
    domain::{
        requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TransactionResponse,
            TransactionResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TransactionQueryService {
    query: DynTransactionQueryRepository,
}

impl TransactionQueryService {
    pub async fn new(query: DynTransactionQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl TransactionQueryServiceTrait for TransactionQueryService {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all transactions | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all transactions: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} transactions", transactions.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError> {
        if req.card_number.trim().is_empty() {
            return Err(ServiceError::Custom("Card number is required".to_string()));
        }

        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "💳 Fetching transactions for card number: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            req.card_number,
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) =
            self.query.find_all_by_card_number(req).await.map_err(|e| {
                error!(
                    "❌ Failed to fetch transactions for card {}: {e:?}",
                    req.card_number,
                );
                ServiceError::Custom(e.to_string())
            })?;

        info!(
            "✅ Found {} transactions for card: {}",
            transactions.len(),
            req.card_number
        );

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by card number retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🟢 Fetching active transactions | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self.query.find_by_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active transactions: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active transactions", transactions.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponseDeleteAt> = transactions
            .into_iter()
            .map(TransactionResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️ Fetching trashed transactions | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transactions, total_items) = self.query.find_by_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed transactions: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed transactions", transactions.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponseDeleteAt> = transactions
            .into_iter()
            .map(TransactionResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError> {
        info!("🔍 Finding transaction by ID: {transaction_id}");

        let transaction = self.query.find_by_id(transaction_id).await.map_err(|e| {
            error!("❌ Database error fetching transaction ID {transaction_id}: {e:?}",);
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found transaction with ID: {transaction_id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transaction retrieved successfully".to_string(),
            data: TransactionResponse::from(transaction),
        })
    }

    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, ServiceError> {
        info!("🏢 Fetching transactions for merchant ID: {merchant_id}");

        let transactions = self
            .query
            .find_by_merchant_id(merchant_id)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch transactions for merchant ID {merchant_id}: {e:?}",);
                ServiceError::Custom(e.to_string())
            })?;

        info!(
            "✅ Found {} transactions for merchant ID: {merchant_id}",
            transactions.len(),
        );

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transactions by merchant ID retrieved successfully".to_string(),
            data: transaction_responses,
        })
    }
}
