use crate::{
    abstract_trait::{
        hashing::DynHashing,
        role::repository::query::DynRoleQueryRepository,
        user::{
            repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
            service::command::UserCommandServiceTrait,
        },
        user_roles::DynUserRoleCommandRepository,
    },
    domain::{
        requests::{
            user::{CreateUserRequest, UpdateUserRequest},
            user_role::CreateUserRoleRequest,
        },
        responses::{ApiResponse, UserResponse, UserResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct UserCommandService {
    query: DynUserQueryRepository,
    command: DynUserCommandRepository,
    hashing: DynHashing,
    user_role: DynUserRoleCommandRepository,
    role: DynRoleQueryRepository,
}

impl UserCommandService {
    pub async fn new(
        query: DynUserQueryRepository,
        command: DynUserCommandRepository,
        hashing: DynHashing,
        user_role: DynUserRoleCommandRepository,
        role: DynRoleQueryRepository,
    ) -> Self {
        Self {
            query,
            command,
            hashing,
            user_role,
            role,
        }
    }
}

#[async_trait]
impl UserCommandServiceTrait for UserCommandService {
    async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating user: {} {}", req.firstname, req.lastname);

        if self
            .query
            .find_by_email(req.email.clone())
            .await
            .map_err(|e| {
                let msg = format!("💥 Failed to check existing email {}: {e:?}", req.email);
                error!("{msg}");
                ServiceError::Custom(msg)
            })?
            .is_some()
        {
            let msg = format!("📧 Email {} already registered", req.email);
            error!("{msg}");
            return Err(ServiceError::Custom(msg));
        }

        let hashed_password = match self.hashing.hash_password(&req.password).await {
            Ok(hash) => hash,
            Err(e) => {
                error!("❌ Failed to hash password: {e:?}");

                return Err(ServiceError::InternalServerError(
                    "Failed to hash password".into(),
                ));
            }
        };

        const DEFAULT_ROLE_NAME: &str = "ROLE_ADMIN";
        let role = match self.role.find_by_name(DEFAULT_ROLE_NAME).await {
            Ok(Some(role)) => role,
            Ok(None) => {
                error!("❌ Role not found: {}", DEFAULT_ROLE_NAME);
                return Err(ServiceError::Custom("Default role not found".to_string()));
            }
            Err(e) => {
                error!("❌ Failed to query role: {e:?}");
                return Err(ServiceError::Repo(e));
            }
        };

        let new_request = &CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            password: hashed_password,
            email: req.email.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        let new_user = self.command.create(new_request).await.map_err(|e| {
            let msg = format!("💥 Failed to create user {}: {e:?}", req.email);
            error!("{msg}");
            ServiceError::Custom(msg)
        })?;

        let assign_role_request = CreateUserRoleRequest {
            user_id: new_user.user_id,
            role_id: role.role_id,
        };

        if let Err(e) = self
            .user_role
            .assign_role_to_user(&assign_role_request)
            .await
        {
            error!(
                "❌ Failed to assign role {} to user {}: {e:?}",
                role.role_id, new_user.user_id,
            );
            return Err(ServiceError::Repo(e));
        }

        let response = UserResponse::from(new_user);

        info!("✅ User created successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ User created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        req.validate().map_err(|e| {
            let msg = format!("❌ Validation failed: {e:?}");
            error!("{msg}");
            ServiceError::Custom(msg)
        })?;

        let user_id = req
            .id
            .ok_or_else(|| ServiceError::Custom("user_id is required".into()))?;
        info!("🔄 Updating user id={user_id}");

        let user = self.query.find_by_id(user_id).await.map_err(|e| {
            let msg = format!("👤 User not found with id {user_id}: {e:?}");
            error!("{msg}");
            ServiceError::Custom(msg)
        })?;

        if let Some(new_email) = &req.email {
            let new_email_norm = new_email.trim().to_lowercase();
            let old_email_norm = user.email.trim().to_lowercase();

            if new_email_norm == old_email_norm {
                info!("📧 Email unchanged after normalization, skipping DB check");
            } else {
                match self.query.find_by_email(new_email_norm.clone()).await {
                    Ok(Some(_)) => {
                        let msg = format!("📧 Email {new_email} already used by another user");
                        error!("{msg}");
                        return Err(ServiceError::Custom(msg));
                    }
                    Ok(None) => {
                        info!("📧 Email {new_email} available for use");
                    }
                    Err(e) => {
                        let msg = format!("💥 Failed to check email {new_email}: {e:?}");
                        error!("{msg}");
                        return Err(ServiceError::Custom(msg));
                    }
                }
            }
        } else {
            info!("📧 No email in request, skip email validation");
        }

        let updated_user = self.command.update(req).await.map_err(|e| {
            let msg = format!("💥 Failed to update user {user_id}: {e:?}");
            error!("{msg}");
            ServiceError::Custom(msg)
        })?;

        let response = UserResponse::from(updated_user);
        info!("✅ User updated successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ User updated successfully!".into(),
            data: response,
        })
    }

    async fn trashed(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing user id={}", user_id);

        match self.query.find_by_id(user_id).await {
            Ok(_) => match self.command.trashed(user_id).await {
                Ok(user) => {
                    let response = UserResponseDeleteAt::from(user);
                    Ok(ApiResponse {
                        status: "success".into(),
                        message: "🗑️ User trashed successfully!".into(),
                        data: response,
                    })
                }
                Err(e) => {
                    let msg = format!("💥 Failed to trash user {user_id}: {e:?}");
                    error!("{msg}");
                    Err(ServiceError::Custom(msg))
                }
            },
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn restore(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring user id={user_id}");

        match self.query.find_by_id(user_id).await {
            Ok(_) => match self.command.restore(user_id).await {
                Ok(user) => {
                    let response = UserResponseDeleteAt::from(user);
                    Ok(ApiResponse {
                        status: "success".into(),
                        message: "♻️ User restored successfully!".into(),
                        data: response,
                    })
                }
                Err(e) => {
                    let msg = format!("💥 Failed to restore user {user_id}: {e:?}");
                    error!("{msg}");
                    Err(ServiceError::Custom(msg))
                }
            },
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting user id={user_id}");

        match self.query.find_by_id(user_id).await {
            Ok(_) => match self.command.delete_permanent(user_id).await {
                Ok(_) => Ok(ApiResponse {
                    status: "success".into(),
                    message: "🧨 User permanently deleted!".into(),
                    data: true,
                }),
                Err(e) => {
                    let msg = format!("💥 Failed to permanently delete user {user_id}: {e:?}");
                    error!("{msg}");
                    Err(ServiceError::Custom(msg))
                }
            },
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed users");

        match self.command.restore_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "🔄 All users restored successfully!".into(),
                data: true,
            }),
            Err(e) => {
                let msg = format!("💥 Failed to restore all users: {e:?}");
                error!("{msg}");
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed users");

        match self.command.delete_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "💣 All users permanently deleted!".into(),
                data: true,
            }),
            Err(e) => {
                let msg = format!("💥 Failed to permanently delete all users: {e:?}");
                error!("{msg}");
                Err(ServiceError::Custom(msg))
            }
        }
    }
}
