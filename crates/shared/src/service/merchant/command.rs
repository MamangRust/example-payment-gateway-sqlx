use crate::{
    abstract_trait::{
        merchant::{
            repository::command::DynMerchantCommandRepository,
            service::command::MerchantCommandServiceTrait,
        },
        user::repository::query::DynUserQueryRepository,
    },
    domain::{
        requests::merchant::{CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus},
        responses::{ApiResponse, MerchantResponse, MerchantResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
    utils::generate_api_key,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct MerchantCommandService {
    command: DynMerchantCommandRepository,
    user_query: DynUserQueryRepository,
}

impl MerchantCommandService {
    pub async fn new(
        command: DynMerchantCommandRepository,
        user_query: DynUserQueryRepository,
    ) -> Self {
        Self {
            command,
            user_query,
        }
    }
}

#[async_trait]
impl MerchantCommandServiceTrait for MerchantCommandService {
    async fn create(
        &self,
        req: &CreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!(
            "🆕 Creating merchant: {} for user_id={}",
            req.name, req.user_id
        );

        let api_key = generate_api_key();

        let _user = self.user_query.find_by_id(req.user_id).await.map_err(|e| {
            let error_msg = format!("👤 User lookup failed for user_id={}: {e:?}", req.user_id);
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        let merchant = self.command.create(api_key, req).await.map_err(|e| {
            let error_msg = format!(
                "💥 Failed to create merchant {} (user_id={}): {e:?}",
                req.name, req.user_id
            );
            error!("{}", error_msg);
            ServiceError::Custom(error_msg)
        })?;

        info!(
            "✅ Merchant created successfully: id={}",
            merchant.merchant_id
        );

        let response = MerchantResponse::from(merchant);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant created successfully".to_string(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🔄 Updating merchant id={}", req.merchant_id);

        let _user = self.user_query.find_by_id(req.user_id).await.map_err(|e| {
            let error_msg = format!(
                "👤 User lookup failed for user_id={} during merchant update: {e:?}",
                req.user_id
            );
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        let updated_merchant = self.command.update(req).await.map_err(|e| {
            let error_msg = format!("💥 Failed to update merchant id={}: {e:?}", req.merchant_id);
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        info!(
            "✅ Merchant updated successfully: id={}",
            updated_merchant.merchant_id
        );

        let response = MerchantResponse::from(updated_merchant);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant updated successfully".to_string(),
            data: response,
        })
    }

    async fn update_status(
        &self,
        req: &UpdateMerchantStatus,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!(
            "🔄 Updating status for merchant id={} to {}",
            req.merchant_id, req.status
        );

        let updated_merchant = self.command.update_status(req).await.map_err(|e| {
            let error_msg = format!(
                "💥 Failed to update status for merchant id={} to {}: {e:?}",
                req.merchant_id, req.status
            );
            error!("{error_msg}");
            ServiceError::Custom(error_msg)
        })?;

        info!(
            "✅ Merchant status updated successfully: id={}, status={}",
            updated_merchant.merchant_id, updated_merchant.status
        );

        let response = MerchantResponse::from(updated_merchant);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant status updated successfully".to_string(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing merchant id={id}");

        match self.command.trash(id).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant trashed successfully: id={}",
                    merchant.merchant_id
                );
                let response = MerchantResponseDeleteAt::from(merchant);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Merchant trashed successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to trash merchant id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to trash merchant".into()))
            }
        }
    }

    async fn restore(
        &self,
        id: i32,
    ) -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring merchant id={id}");

        match self.command.restore(id).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant restored successfully: id={}",
                    merchant.merchant_id
                );
                let response = MerchantResponseDeleteAt::from(merchant);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Merchant restored successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore merchant id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to restore merchant".into()))
            }
        }
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting merchant id={id}");

        match self.command.delete_permanent(id).await {
            Ok(_) => {
                info!("✅ Merchant permanently deleted: id={id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: format!("Merchant with id={id} permanently deleted"),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete merchant id={id}: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete merchant".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed merchants");

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All merchants restored successfully");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All trashed merchants restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all merchants: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to restore all merchants".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed merchants");

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All merchants permanently deleted");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All trashed merchants permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to delete all merchants: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to delete all merchants".into(),
                ))
            }
        }
    }
}
