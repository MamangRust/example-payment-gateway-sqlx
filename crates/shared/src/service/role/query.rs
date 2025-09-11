use crate::{
    abstract_trait::role::{
        repository::query::DynRoleQueryRepository, service::query::RoleQueryServiceTrait,
    },
    domain::{
        requests::role::FindAllRoles,
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, RoleResponse, RoleResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct RoleQueryService {
    query: DynRoleQueryRepository,
}

impl RoleQueryService {
    pub async fn new(query: DynRoleQueryRepository) -> Self {
        Self { query }
    }
}

#[async_trait]
impl RoleQueryServiceTrait for RoleQueryService {
    async fn find_all(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        info!(
            "🔍 Searching all roles | Page: {page_size}, Size: {page_size}, Search: {:?}",
            search.as_deref().unwrap_or("None")
        );

        let (roles, total_items) = self.query.find_all(request).await.map_err(|e| {
            error!("❌ Failed to fetch all roles: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} roles", roles.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponse> = roles.into_iter().map(RoleResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Roles retrieved successfully".to_string(),
            data: role_responses,
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
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        info!(
            "🔍 Searching active roles | Page: {page}, Size: {page_size}, Search: {}",
            search.as_deref().unwrap_or("None")
        );

        let (roles, total_items) = self.query.find_active(request).await.map_err(|e| {
            error!("❌ Failed to fetch active roles: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} active roles", roles.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponseDeleteAt> =
            roles.into_iter().map(RoleResponseDeleteAt::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Active roles retrieved successfully".to_string(),
            data: role_responses,
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
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError> {
        let page = if request.page > 0 { request.page } else { 1 };
        let page_size = if request.page_size > 0 {
            request.page_size
        } else {
            10
        };
        let search = if request.search.is_empty() {
            None
        } else {
            Some(request.search.clone())
        };

        info!(
            "🔍 Searching trashed roles | Page: {page}, Size: {page_size}, Search: {}",
            search.as_deref().unwrap_or("None")
        );

        let (roles, total_items) = self.query.find_trashed(request).await.map_err(|e| {
            error!("❌ Failed to fetch trashed roles: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} trashed roles", roles.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let role_responses: Vec<RoleResponseDeleteAt> =
            roles.into_iter().map(RoleResponseDeleteAt::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed roles retrieved successfully".to_string(),
            data: role_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        info!("🔍 Finding role by ID: {id}");

        let role = self
            .query
            .find_by_id(id)
            .await
            .map_err(|e| {
                error!("❌ Database error while finding role by ID {id}: {e:?}");
                ServiceError::Custom(e.to_string())
            })?
            .ok_or_else(|| {
                error!("❌ Role with ID {id} not found");
                ServiceError::NotFound(format!("Role with ID {id} not found"))
            })?;

        info!("✅ Found role with ID: {id}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Role retrieved successfully".to_string(),
            data: RoleResponse::from(role),
        })
    }
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, ServiceError> {
        info!("🔍 Finding roles for user ID: {user_id}");

        let roles = self.query.find_by_user_id(user_id).await.map_err(|e| {
            error!("❌ Failed to fetch roles for user ID {user_id}: {e:?}");
            ServiceError::Custom(e.to_string())
        })?;

        info!("✅ Found {} roles for user ID: {user_id}", roles.len());

        let role_responses: Vec<RoleResponse> = roles.into_iter().map(RoleResponse::from).collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User roles retrieved successfully".to_string(),
            data: role_responses,
        })
    }
    async fn find_by_name(&self, name: String) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        info!("🔍 Finding role by name: {name}");

        let role = self
            .query
            .find_by_name(&name)
            .await
            .map_err(|e| {
                error!("❌ Database error while finding role by name '{name}': {e:?}",);
                ServiceError::Custom(e.to_string())
            })?
            .ok_or_else(|| {
                error!("❌ Role with name '{name}' not found");
                ServiceError::NotFound(format!("Role with name '{name}' not found"))
            })?;

        info!("✅ Found role with name: {name}");

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Role retrieved by name successfully".to_string(),
            data: RoleResponse::from(role),
        })
    }
}
