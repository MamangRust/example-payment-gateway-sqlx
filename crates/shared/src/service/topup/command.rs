use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        topup::{
            repository::{command::DynTopupCommandRepository, query::DynTopupQueryRepository},
            service::command::TopupCommandServiceTrait,
        },
    },
    domain::requests::{
        saldo::UpdateSaldoBalance,
        topup::{CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest, UpdateTopupStatus},
    },
    domain::responses::{ApiResponse, TopupResponse, TopupResponseDeleteAt},
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TopupCommandService {
    card_query: DynCardQueryRepository,
    saldo_query: DynSaldoQueryRepository,
    saldo_command: DynSaldoCommandRepository,
    query: DynTopupQueryRepository,
    command: DynTopupCommandRepository,
}

impl TopupCommandService {
    pub async fn new(
        card_query: DynCardQueryRepository,
        saldo_query: DynSaldoQueryRepository,
        saldo_command: DynSaldoCommandRepository,
        query: DynTopupQueryRepository,
        command: DynTopupCommandRepository,
    ) -> Self {
        Self {
            card_query,
            saldo_query,
            saldo_command,
            query,
            command,
        }
    }
}

#[async_trait]
impl TopupCommandServiceTrait for TopupCommandService {
    async fn create(
        &self,
        req: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🚀 Starting CreateTopup: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let _card = match self.card_query.find_by_card(&req.card_number).await {
            Ok(c) => c,
            Err(e) => {
                error!("❌ Failed to find card by number: {e:?}");
                return Err(ServiceError::Custom("card not found".into()));
            }
        };

        let topup = match self.command.create(req).await {
            Ok(t) => t,
            Err(e) => {
                error!("❌ Failed to create topup: {e:?}");
                return Err(ServiceError::Custom("failed to create topup".into()));
            }
        };

        let mut saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(s) => s,
            Err(e) => {
                error!("❌ Failed to find saldo: {e:?}");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id: topup.topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                return Err(ServiceError::Custom("saldo not found".into()));
            }
        };

        let new_balance = saldo.total_balance + req.topup_amount;
        saldo.total_balance = new_balance;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: req.card_number.clone(),
                total_balance: new_balance,
            })
            .await
        {
            error!("❌ Failed to update saldo: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id: topup.topup_id,
                    status: "failed".to_string(),
                })
                .await;
            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        if let Err(e) = self
            .command
            .update_status(&UpdateTopupStatus {
                topup_id: topup.topup_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("❌ Failed to update topup status: {e:?}");
            return Err(ServiceError::Custom("failed to update topup status".into()));
        }

        let response = TopupResponse::from(topup);

        info!(
            "✅ CreateTopup completed: card={} topup_amount={} new_balance={new_balance}",
            req.card_number, req.topup_amount
        );

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup berhasil diproses".into(),
            data: response,
        })
    }
    async fn update(
        &self,
        req: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🚀 Starting UpdateTopup: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let topup_id = req
            .topup_id
            .ok_or_else(|| ServiceError::Custom("topup_id is required".into()))?;

        if let Err(e) = self.card_query.find_by_card(&req.card_number).await {
            error!("❌ Card not found: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id: topup_id,
                    status: "failed".to_string(),
                })
                .await;
            return Err(ServiceError::Custom("card not found".into()));
        }

        let existing = match self.query.find_by_id(topup_id).await {
            Ok(t) => t,
            _ => {
                error!("❌ Topup {topup_id} not found");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id: topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                return Err(ServiceError::Custom("topup not found".into()));
            }
        };

        let difference = req.topup_amount - existing.topup_amount;

        if let Err(e) = self.command.update(req).await {
            error!("❌ Failed to update topup: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id: topup_id,
                    status: "failed".to_string(),
                })
                .await;
            return Err(ServiceError::Custom("failed to update topup".into()));
        }

        let mut saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(s) => s,
            Err(e) => {
                error!("❌ Failed to get saldo: {e:?}");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id: topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                return Err(ServiceError::Custom("saldo not found".into()));
            }
        };

        let new_balance = saldo.total_balance + difference;
        saldo.total_balance = new_balance;

        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: req.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
        {
            error!("❌ Failed to update saldo: {e:?}");

            let _ = self
                .command
                .update_amount(&UpdateTopupAmount {
                    topup_id: topup_id,
                    topup_amount: existing.topup_amount,
                })
                .await;
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id: topup_id,
                    status: "failed".to_string(),
                })
                .await;

            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        let updated_topup = match self.query.find_by_id(topup_id).await {
            Ok(t) => t,
            _ => {
                error!("❌ Failed to fetch updated topup {topup_id}");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id: topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                return Err(ServiceError::Custom("topup not found".into()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTopupStatus {
                topup_id: topup_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("❌ Failed to update topup status: {e:?}");
            return Err(ServiceError::Custom("failed to update topup status".into()));
        }

        let response = TopupResponse::from(updated_topup);

        info!(
            "✅ UpdateTopup completed: card={} topup_id={} new_amount={} new_balance={new_balance}",
            req.card_number, topup_id, req.topup_amount,
        );

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup berhasil diperbarui".into(),
            data: response,
        })
    }
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing topup id={topup_id}");

        match self.command.trashed(topup_id).await {
            Ok(topup) => {
                info!("✅ Topup trashed successfully: id={}", topup_id);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Topup trashed successfully".into(),
                    data: TopupResponseDeleteAt::from(topup),
                })
            }
            Err(e) => {
                error!("💥 Failed to trash topup id={topup_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to trash topup with id {topup_id}"
                )))
            }
        }
    }

    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring topup id={topup_id}");

        match self.command.restore(topup_id).await {
            Ok(topup) => {
                info!("✅ Topup restored successfully: id={}", topup_id);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Topup restored successfully".into(),
                    data: TopupResponseDeleteAt::from(topup),
                })
            }
            Err(e) => {
                error!("💥 Failed to restore topup id={topup_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to restore topup with id {topup_id}"
                )))
            }
        }
    }

    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting topup id={topup_id}");

        match self.command.delete_permanent(topup_id).await {
            Ok(_) => {
                info!("✅ Topup permanently deleted: id={topup_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Topup permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete topup id={topup_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete topup with id {topup_id}"
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring all trashed topups");

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All topups restored successfully");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All topups restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all topups: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to restore all trashed topups".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting all trashed topups");

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All topups permanently deleted");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All topups permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete all topups: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed topups".into(),
                ))
            }
        }
    }
}
