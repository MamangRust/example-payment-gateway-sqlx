use crate::{
    abstract_trait::transfer::{
        repository::query::DynTransferQueryRepository, service::query::TransferQueryServiceTrait,
    },
    domain::{
        requests::transfer::FindAllTransfers,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TransferResponse,
            TransferResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TransferQueryService {
    query: DynTransferQueryRepository,
}

impl TransferQueryService {
    pub async fn new(query: DynTransferQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl TransferQueryServiceTrait for TransferQueryService {
    async fn find_all(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all transfers | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transfers, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all transfers: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} transfers", transfers.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Transfers retrieved successfully".to_string(),
            data: transfer_responses,
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
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError> {
        info!("🔍 Finding transfer by ID: {}", transfer_id);

        let transfer = self.query.find_by_id(transfer_id).await.map_err(|e| {
            error!("❌ Database error fetching transfer ID {transfer_id}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found transfer with ID: {transfer_id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer retrieved successfully".to_string(),
            data: TransferResponse::from(transfer),
        })
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all transfers active | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transfers, total_items) = self.query.find_by_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active transfers: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active transfers", transfers.len());
        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponseDeleteAt> = transfers
            .into_iter()
            .map(TransferResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active transfers retrieved successfully".to_string(),
            data: transfer_responses,
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
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all transfers trashed | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (transfers, total_items) = self.query.find_by_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed transfers: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed transfers", transfers.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transfer_responses: Vec<TransferResponseDeleteAt> = transfers
            .into_iter()
            .map(TransferResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed transfers retrieved successfully".to_string(),
            data: transfer_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError> {
        if transfer_from.to_string().trim().is_empty() {
            return Err(ServiceError::Custom(
                "Transfer from account is required".to_string(),
            ));
        }

        info!("📤 Fetching transfers sent from: {transfer_from}");

        let transfers = self
            .query
            .find_by_transfer_from(transfer_from)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch transfers from {transfer_from}: {e:?}");
                ServiceError::Custom(e.to_string())
            })?;

        info!(
            "✅ Found {} transfers sent from: {transfer_from}",
            transfers.len(),
        );

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfers from account retrieved successfully".to_string(),
            data: transfer_responses,
        })
    }

    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError> {
        if transfer_to.to_string().trim().is_empty() {
            return Err(ServiceError::Custom(
                "Transfer to account is required".to_string(),
            ));
        }

        info!("📥 Fetching transfers sent to: {transfer_to}");

        let transfers = self
            .query
            .find_by_transfer_to(transfer_to)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch transfers to {transfer_to}: {e:?}");
                ServiceError::Custom(e.to_string())
            })?;

        info!(
            "✅ Found {} transfers sent to: {transfer_to}",
            transfers.len(),
        );

        let transfer_responses: Vec<TransferResponse> =
            transfers.into_iter().map(TransferResponse::from).collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfers to account retrieved successfully".to_string(),
            data: transfer_responses,
        })
    }
}
