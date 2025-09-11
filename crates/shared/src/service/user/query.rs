use crate::{
    abstract_trait::user::{
        repository::query::DynUserQueryRepository, service::query::UserQueryServiceTrait,
    },
    domain::{
        requests::user::FindAllUserRequest,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, UserResponse, UserResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct UserQueryService {
    query: DynUserQueryRepository,
}

impl UserQueryService {
    pub async fn new(query: DynUserQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl UserQueryServiceTrait for UserQueryService {
    async fn find_all(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🔍 Searching all users | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (users, total_items) = self.query.find_all(req.clone()).await.map_err(|e| {
            error!("❌ Failed to fetch all users: {}", e);
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} users", users.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Users retrieved successfully".to_string(),
            data: user_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!("🔍 Finding user by ID: {user_id}");

        let user = self.query.find_by_id(user_id).await.map_err(|e| {
            error!("❌ Database error fetching user ID {user_id}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found user with ID: {user_id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User retrieved successfully".to_string(),
            data: UserResponse::from(user),
        })
    }

    async fn find_by_active(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🟢 Fetching active users | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (users, total_items) = self.query.find_by_active(req.clone()).await.map_err(|e| {
            error!("❌ Failed to fetch active users: {}", e);
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active users", users.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponseDeleteAt> =
            users.into_iter().map(UserResponseDeleteAt::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active users retrieved successfully".to_string(),
            data: user_responses,
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
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        info!(
            "🗑️ Fetching trashed users | Page: {page}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (users, total_items) = self.query.find_by_trashed(req.clone()).await.map_err(|e| {
            error!("❌ Failed to fetch trashed users: {}", e);
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed users", users.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let user_responses: Vec<UserResponseDeleteAt> =
            users.into_iter().map(UserResponseDeleteAt::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed users retrieved successfully".to_string(),
            data: user_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
}
