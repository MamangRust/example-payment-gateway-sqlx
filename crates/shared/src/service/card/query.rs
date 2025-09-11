use crate::{
    abstract_trait::card::{
        repository::query::DynCardQueryRepository, service::query::CardQueryServiceTrait,
    },
    domain::{
        requests::card::FindAllCards,
        responses::{
            ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt, Pagination,
        },
    },
    errors::{RepositoryError, ServiceError},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardQueryService {
    query: DynCardQueryRepository,
}

impl CardQueryService {
    pub async fn new(query: DynCardQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl CardQueryServiceTrait for CardQueryService {
    async fn find_all(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (cards, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all cards: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} cards", cards.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponse> = cards.into_iter().map(CardResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Cards retrieved successfully".to_string(),
            data: card_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_active(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Fetching active cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (cards, total_items) = self.query.find_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active cards: {e:?}");
            ServiceError::InternalServerError(e.to_string())
        })?;

        info!("✅ Retrieved {} active cards", cards.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponseDeleteAt> =
            cards.into_iter().map(|c| c.into()).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active cards retrieved successfully".to_string(),
            data: card_responses,
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
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️  Fetching trashed cards | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (cards, total_items) = self.query.find_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed cards: {e:?}");
            ServiceError::InternalServerError(e.to_string())
        })?;

        info!("🗑️  Found {} trashed cards", cards.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let card_responses: Vec<CardResponseDeleteAt> =
            cards.into_iter().map(|c| c.into()).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed cards retrieved successfully".to_string(),
            data: card_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!("🔍 Finding card by ID: {id}");

        let card = self.query.find_by_id(id).await.map_err(|e| match e {
            RepositoryError::NotFound => {
                info!("ℹ️  Card with ID {id} not found");
                ServiceError::NotFound("Card not found".to_string())
            }
            _ => {
                error!("❌ Database error while finding card by ID {id}: {e:?}");
                ServiceError::InternalServerError(e.to_string())
            }
        })?;

        info!("✅ Card retrieved successfully (ID: {id})");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Card retrieved successfully".to_string(),
            data: CardResponse::from(card),
        })
    }

    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!("👥 Finding card for user ID: {}", user_id);

        let card = self.query.find_by_user_id(user_id).await.map_err(|e| {
            error!("❌ Failed to fetch card for user ID {user_id}: {e:?}");
            ServiceError::InternalServerError(e.to_string())
        })?;

        let response_data = CardResponse::from(card);

        info!("✅ Found card for user ID {user_id}");

        Ok(ApiResponse {
            status: "success".into(),
            message: "Card by user ID retrieved successfully".into(),
            data: response_data,
        })
    }

    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        info!(
            "💳 Finding card by card number: {}",
            mask_card_number(card_number)
        );

        let card = self
            .query
            .find_by_card(card_number)
            .await
            .map_err(|e| match e {
                RepositoryError::NotFound => {
                    info!(
                        "ℹ️  Card with number {} not found",
                        mask_card_number(card_number)
                    );
                    ServiceError::NotFound("Card not found".to_string())
                }
                _ => {
                    error!(
                        "❌ Error fetching card by number {}: {e:?}",
                        mask_card_number(card_number),
                    );
                    ServiceError::InternalServerError(e.to_string())
                }
            })?;

        info!(
            "✅ Card with number {} retrieved successfully",
            mask_card_number(&card.card_number)
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Card retrieved by card number".to_string(),
            data: CardResponse::from(card),
        })
    }
}
