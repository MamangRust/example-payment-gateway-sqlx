use crate::{
    abstract_trait::role::{
        repository::command::DynRoleCommandRepository, service::command::RoleCommandServiceTrait,
    },
    domain::{
        requests::role::{CreateRoleRequest, UpdateRoleRequest},
        responses::{ApiResponse, RoleResponse, RoleResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct RoleCommandService {
    command: DynRoleCommandRepository,
}

impl RoleCommandService {
    pub async fn new(command: DynRoleCommandRepository) -> Self {
        Self { command }
    }
}

#[async_trait]
impl RoleCommandServiceTrait for RoleCommandService {
    async fn create(
        &self,
        req: &CreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating role with name: {}", req.name);

        let role = self.command.create(req).await.map_err(|e| {
            let error_msg = format!("💥 Failed to create role with name {}: {e:?}", req.name);
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        let response = RoleResponse::from(role);

        info!("✅ Role created successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Role created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🔄 Updating role id={}", req.id);

        let updated_role = self.command.update(req).await.map_err(|e| {
            let error_msg = format!("💥 Failed to update role id={}: {e:?}", req.id);
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        let response = RoleResponse::from(updated_role);

        info!("✅ Role updated successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Role updated successfully!".into(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing role id={id}");

        match self.command.trash(id).await {
            Ok(role) => {
                let response = RoleResponseDeleteAt::from(role);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🗑️ Role trashed successfully!".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to trash role id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to trash role".into()))
            }
        }
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring role id={id}");

        match self.command.restore(id).await {
            Ok(role) => {
                let response = RoleResponseDeleteAt::from(role);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "♻️ Role restored successfully!".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore role id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to restore role".into()))
            }
        }
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting role id={id}");

        match self.command.delete_permanent(id).await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "🧨 Role permanently deleted!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to permanently delete role id={id}: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete role".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed roles");

        match self.command.restore_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "🔄 All roles restored successfully!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to restore all roles: {e:?}");
                Err(ServiceError::Custom("Failed to restore all roles".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed roles");

        match self.command.delete_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "💣 All roles permanently deleted!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to delete all roles: {e:?}");
                Err(ServiceError::Custom("Failed to delete all roles".into()))
            }
        }
    }
}
