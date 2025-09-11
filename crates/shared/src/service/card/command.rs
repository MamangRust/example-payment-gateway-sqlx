use crate::{
    abstract_trait::{
        card::{
            repository::{command::DynCardCommandRepository, query::DynCardQueryRepository},
            service::command::CardCommandServiceTrait,
        },
        user::repository::query::DynUserQueryRepository,
    },
    domain::{
        requests::card::{CreateCardRequest, UpdateCardRequest},
        responses::{ApiResponse, CardResponse, CardResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct CardCommandService {
    user_query: DynUserQueryRepository,
    command: DynCardCommandRepository,
}

impl CardCommandService {
    pub async fn new(
        user_query: DynUserQueryRepository,
        command: DynCardCommandRepository,
    ) -> Self {
        Self {
            command,
            user_query,
        }
    }
}

#[async_trait]
impl CardCommandServiceTrait for CardCommandService {
    async fn create(
        &self,
        req: &CreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating card for user_id={}", req.user_id);

        let _user = self.user_query.find_by_id(req.user_id).await.map_err(|e| {
            error!("👤 Failed to find user with id {}: {e:?}", req.user_id);
            ServiceError::Custom("Failed to fetch user".into())
        })?;

        let card = self.command.create(req).await.map_err(|e| {
            error!(
                "💥 Failed to create card for user_id {}: {e:?}",
                req.user_id,
            );
            ServiceError::Custom("Failed to create card".into())
        })?;

        let response = CardResponse::from(card);

        info!("✅ Card created successfully with card_id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Card created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!(
            "🔄 Updating card id={} for user_id={}",
            req.card_id, req.user_id
        );

        let _user = self.user_query.find_by_id(req.user_id).await.map_err(|e| {
            error!(
                "👤 Failed to find user with id {} during update: {e:?}",
                req.user_id,
            );
            ServiceError::Custom("Failed to fetch user".into())
        })?;

        let updated_card = self.command.update(req).await.map_err(|e| {
            error!("💥 Failed to update card id {}: {e:?}", req.card_id);
            ServiceError::Custom("Failed to update card".into())
        })?;

        let response = CardResponse::from(updated_card);

        info!("✅ Card updated successfully with card_id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Card updated successfully!".into(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing card id={id}");

        match self.command.trash(id).await {
            Ok(card) => {
                let response = CardResponseDeleteAt::from(card);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🗑️ Card trashed successfully!".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to trash card id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to trash card".into()))
            }
        }
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring card id={id}");

        match self.command.restore(id).await {
            Ok(card) => {
                let response = CardResponseDeleteAt::from(card);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "♻️ Card restored successfully!".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore card id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to restore card".into()))
            }
        }
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting card id={id}");

        match self.command.delete_permanent(id).await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "🧨 Card permanently deleted!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to permanently delete card id={id}: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete card".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed cards");

        match self.command.restore_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "🔄 All cards restored successfully!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to restore all cards: {e:?}");
                Err(ServiceError::Custom("Failed to restore all cards".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed cards");

        match self.command.delete_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "💣 All cards permanently deleted!".into(),
                data: true,
            }),
            Err(e) => {
                error!("💥 Failed to delete all cards: {e:?}");
                Err(ServiceError::Custom("Failed to delete all cards".into()))
            }
        }
    }
}
