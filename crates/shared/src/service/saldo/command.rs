use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::{
            repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
            service::command::SaldoCommandServiceTrait,
        },
    },
    domain::{
        requests::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
        responses::{ApiResponse, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct SaldoCommandService {
    command: DynSaldoCommandRepository,
    card_query: DynCardQueryRepository,
}

impl SaldoCommandService {
    pub async fn new(
        command: DynSaldoCommandRepository,
        card_query: DynCardQueryRepository,
    ) -> Self {
        Self {
            command,
            card_query,
        }
    }
}

#[async_trait]
impl SaldoCommandServiceTrait for SaldoCommandService {
    async fn create(
        &self,
        request: &CreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        if let Err(validation_errors) = request.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("Creating saldo for card_number={}", request.card_number);

        let _card = self
            .card_query
            .find_by_card(&request.card_number)
            .await
            .map_err(|e| {
                error!("Failed to find card {}: {e:?}", request.card_number);
                ServiceError::Custom("Card not found".into())
            })?;

        let saldo = self.command.create(request).await.map_err(|e| {
            error!(
                "Failed to create saldo for card {}: {e:?}",
                request.card_number,
            );
            ServiceError::Custom("Failed to create saldo".into())
        })?;

        let response = SaldoResponse::from(saldo);

        info!(
            "Saldo created successfully with id={} for card={}",
            response.id, request.card_number
        );

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo created successfully".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        request: &UpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        if let Err(validation_errors) = request.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!(
            "Updating saldo id={} for card={}",
            request.saldo_id, request.card_number
        );

        let _ = self
            .card_query
            .find_by_card(&request.card_number)
            .await
            .map_err(|e| {
                error!(
                    "Failed to find card {} during update: {e:?}",
                    request.card_number,
                );
                ServiceError::Custom("Card not found".into())
            })?;

        let updated_saldo = self.command.update(request).await.map_err(|e| {
            error!(
                "Failed to update saldo id={} for card {}: {e:?}",
                request.saldo_id, request.card_number,
            );
            ServiceError::Custom("Failed to update saldo".into())
        })?;

        let response = SaldoResponse::from(updated_saldo);

        info!(
            "Saldo updated successfully with id={} for card={}",
            response.id, request.card_number
        );

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo updated successfully".into(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing saldo with id={id}");

        match self.command.trash(id).await {
            Ok(saldo) => {
                let response = SaldoResponseDeleteAt::from(saldo);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Saldo trashed successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("❌ Failed to trash saldo with id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to trash saldo".into()))
            }
        }
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring saldo with id={id}");

        match self.command.restore(id).await {
            Ok(saldo) => {
                let response = SaldoResponseDeleteAt::from(saldo);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Saldo restored successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("❌ Failed to restore saldo with id={id}: {e:?}");
                Err(ServiceError::Custom("Failed to restore saldo".into()))
            }
        }
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💀 Permanently deleting saldo with id={id}");

        match self.command.delete_permanent(id).await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "Saldo permanently deleted".into(),
                data: true,
            }),
            Err(e) => {
                error!("❌ Failed to permanently delete saldo with id={id}: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete saldo".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("♻️ Restoring all trashed saldos");

        match self.command.restore_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "All saldos restored successfully".into(),
                data: true,
            }),
            Err(e) => {
                error!("❌ Failed to restore all saldos: {e:?}");
                Err(ServiceError::Custom("Failed to restore all saldos".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💀 Permanently deleting all trashed saldos");

        match self.command.delete_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "All saldos permanently deleted".into(),
                data: true,
            }),
            Err(e) => {
                error!("❌ Failed to permanently delete all saldos: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete all saldos".into(),
                ))
            }
        }
    }
}
