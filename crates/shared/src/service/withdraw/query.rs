use crate::{
    abstract_trait::withdraw::{
        repository::query::DynWithdrawQueryRepository, service::query::WithdrawQueryServiceTrait,
    },
    domain::{
        requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, WithdrawResponse,
            WithdrawResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct WithdrawQueryService {
    query: DynWithdrawQueryRepository,
}

impl WithdrawQueryService {
    pub async fn new(query: DynWithdrawQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl WithdrawQueryServiceTrait for WithdrawQueryService {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all withdrawals | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (withdraws, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all withdrawals: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} withdrawals", withdraws.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponse> =
            withdraws.into_iter().map(WithdrawResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
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
        req: &FindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "💳 Fetching withdrawals for card number: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            req.card_number,
            search.as_deref().unwrap_or("None")
        );

        let (withdraws, total_items) =
            self.query.find_all_by_card_number(req).await.map_err(|e| {
                error!(
                    "❌ Failed to fetch withdrawals for card {}: {e:?}",
                    req.card_number,
                );
                ServiceError::Custom(e.to_string())
            })?;

        info!(
            "✅ Found {} withdrawals for card: {}",
            withdraws.len(),
            req.card_number
        );

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponse> =
            withdraws.into_iter().map(WithdrawResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Withdrawals by card number retrieved successfully".to_string(),
            data: withdraw_responses,
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
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        info!("🔍 Finding withdrawal by ID: {withdraw_id}");

        let withdraw = self.query.find_by_id(withdraw_id).await.map_err(|e| {
            error!("❌ Database error fetching withdrawal ID {withdraw_id}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found withdrawal with ID: {withdraw_id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdrawal retrieved successfully".to_string(),
            data: WithdrawResponse::from(withdraw),
        })
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<WithdrawResponse>>, ServiceError> {
        info!("🔍 Finding withdrawals by card_number: {card_number}");

        let withdrawals = self.query.find_by_card(card_number).await.map_err(|e| {
            error!("❌ Database error fetching withdrawals for card_number {card_number}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!(
            "✅ Found {} withdrawals for card_number: {card_number}",
            withdrawals.len()
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdrawals retrieved successfully".to_string(),
            data: withdrawals
                .into_iter()
                .map(WithdrawResponse::from)
                .collect(),
        })
    }

    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🟢 Fetching active withdrawals | Page: {page}, Size: {page_size}, Search: {}",
            search.as_deref().unwrap_or("None")
        );

        let (withdraws, total_items) = self.query.find_by_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active withdrawals: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active withdrawals", withdraws.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponseDeleteAt> = withdraws
            .into_iter()
            .map(WithdrawResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
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
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️ Fetching trashed withdrawals | Page: {page}, Size: {page_size}, Search: {}",
            search.as_deref().unwrap_or("None")
        );

        let (withdraws, total_items) = self.query.find_by_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed withdrawals: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed withdrawals", withdraws.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let withdraw_responses: Vec<WithdrawResponseDeleteAt> = withdraws
            .into_iter()
            .map(WithdrawResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed withdrawals retrieved successfully".to_string(),
            data: withdraw_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
}
