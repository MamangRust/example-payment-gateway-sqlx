use crate::{
    abstract_trait::topup::{
        repository::query::DynTopupQueryRepository, service::query::TopupQueryServiceTrait,
    },
    domain::{
        requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TopupResponse, TopupResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TopupQueryService {
    query: DynTopupQueryRepository,
}

impl TopupQueryService {
    pub async fn new(query: DynTopupQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl TopupQueryServiceTrait for TopupQueryService {
    async fn find_all(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all topups | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (topups, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all topups: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} topups", topups.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponse> =
            topups.into_iter().map(TopupResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Topups retrieved successfully".to_string(),
            data: topup_responses,
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
        req: &FindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError> {
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
            "💳 Searching topups by card number: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            req.card_number,
            search.as_deref().unwrap_or("None")
        );

        let (topups, total_items) = self.query.find_all_by_card_number(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch topups for card number {}: {e:?}",
                req.card_number,
            );
            ServiceError::Custom(e.to_string())
        })?;

        info!(
            "✅ Found {} topup records for card number: {}",
            topups.len(),
            req.card_number
        );

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponse> =
            topups.into_iter().map(TopupResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Topups by card number retrieved successfully".to_string(),
            data: topup_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🔍 Finding topup by ID: {topup_id}");

        let topup = self.query.find_by_id(topup_id).await.map_err(|e| {
            error!("❌ Database error while finding topup ID {topup_id}: {e:?}",);
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found topup with ID: {topup_id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topup retrieved successfully".to_string(),
            data: TopupResponse::from(topup),
        })
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, ServiceError> {
        info!("🔍 Finding topups by card_number: {card_number}");

        let topups = self.query.find_by_card(card_number).await.map_err(|e| {
            error!("❌ Database error while finding topups for card_number {card_number}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!(
            "✅ Found {} topups for card_number: {card_number}",
            topups.len()
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topups retrieved successfully".to_string(),
            data: topups.into_iter().map(TopupResponse::from).collect(),
        })
    }

    async fn find_active(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🟢 Searching active topups | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (topups, total_items) = self.query.find_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active topups: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active topups", topups.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponseDeleteAt> = topups
            .into_iter()
            .map(TopupResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active topups retrieved successfully".to_string(),
            data: topup_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_trashed(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️ Searching trashed topups | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (topups, total_items) = self.query.find_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed topups: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed topups", topups.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let topup_responses: Vec<TopupResponseDeleteAt> = topups
            .into_iter()
            .map(TopupResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed topups retrieved successfully".to_string(),
            data: topup_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
}
