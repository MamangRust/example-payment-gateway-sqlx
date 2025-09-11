use crate::{
    abstract_trait::merchant::{
        repository::query::DynMerchantQueryRepository, service::query::MerchantQueryServiceTrait,
    },
    domain::{
        requests::merchant::FindAllMerchants,
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
            Pagination,
        },
    },
    errors::{RepositoryError, ServiceError},
    utils::mask_api_key,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantQueryService {
    query: DynMerchantQueryRepository,
}

impl MerchantQueryService {
    pub async fn new(query: DynMerchantQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl MerchantQueryServiceTrait for MerchantQueryService {
    async fn find_all(
        &self,
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (merchants, total_items) = self.query.find_all(req).await.map_err(|e| {
            error!("❌ Failed to fetch all merchants: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} merchants", merchants.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponse> =
            merchants.into_iter().map(MerchantResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Merchants retrieved successfully".to_string(),
            data: merchant_responses,
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
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "✅ Fetching active merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (merchants, total_items) = self.query.find_active(req).await.map_err(|e| {
            error!("❌ Failed to fetch active merchants: {e:?}");
            ServiceError::InternalServerError(e.to_string())
        })?;

        info!("✅ Retrieved {} active merchants", merchants.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponseDeleteAt> = merchants
            .into_iter()
            .map(MerchantResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active merchants retrieved successfully".to_string(),
            data: merchant_responses,
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
        req: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️  Fetching trashed merchants | Page: {}, Size: {}, Search: {:?}",
            page,
            page_size,
            search.as_deref().unwrap_or("None")
        );

        let (merchants, total_items) = self.query.find_trashed(req).await.map_err(|e| {
            error!("❌ Failed to fetch trashed merchants: {e:?}");
            ServiceError::InternalServerError(e.to_string())
        })?;

        info!("🗑️  Found {} trashed merchants", merchants.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let merchant_responses: Vec<MerchantResponseDeleteAt> = merchants
            .into_iter()
            .map(MerchantResponseDeleteAt::from)
            .collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed merchants retrieved successfully".to_string(),
            data: merchant_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        info!("🔍 Finding merchant by ID: {}", id);

        let merchant = self.query.find_by_id(id).await.map_err(|e| match e {
            RepositoryError::NotFound => {
                info!("ℹ️  Merchant with ID {id} not found");
                ServiceError::NotFound("Merchant not found".to_string())
            }
            _ => {
                error!("❌ Database error while finding merchant by ID {id}: {e:?}",);
                ServiceError::InternalServerError(e.to_string())
            }
        })?;

        info!("✅ Merchant retrieved successfully (ID: {id})");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant retrieved successfully".to_string(),
            data: MerchantResponse::from(merchant),
        })
    }

    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        let masked_key = mask_api_key(&api_key);

        info!("🔑 Finding merchant by API key: {masked_key}");

        let merchant = self
            .query
            .find_by_apikey(api_key)
            .await
            .map_err(|e| match e {
                RepositoryError::NotFound => {
                    info!("ℹ️  No merchant found for API key: {masked_key}");
                    ServiceError::NotFound("Invalid API key".to_string())
                }
                _ => {
                    error!("❌ Error fetching merchant by API key {masked_key}: {e:?}",);
                    ServiceError::InternalServerError(e.to_string())
                }
            })?;

        info!("✅ Merchant found for API key: {masked_key}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant retrieved by API key".to_string(),
            data: MerchantResponse::from(merchant),
        })
    }

    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError> {
        info!("👥 Finding merchants for user ID: {user_id}");

        let merchants = self
            .query
            .find_merchant_user_id(user_id)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch merchants for user ID {user_id}: {e:?}",);
                ServiceError::InternalServerError(e.to_string())
            })?;

        info!(
            "✅ Found {} merchants for user ID {user_id}",
            merchants.len(),
        );

        let merchant_responses: Vec<MerchantResponse> =
            merchants.into_iter().map(MerchantResponse::from).collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchants by user ID retrieved successfully".to_string(),
            data: merchant_responses,
        })
    }
}
