use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        withdraw::{
            repository::{
                command::DynWithdrawCommandRepository, query::DynWithdrawQueryRepository,
            },
            service::command::WithdrawCommandServiceTrait,
        },
    },
    domain::{
        requests::{
            saldo::UpdateSaldoWithdraw,
            withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest, UpdateWithdrawStatus},
        },
        responses::{ApiResponse, WithdrawResponse, WithdrawResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct WithdrawCommandService {
    query: DynWithdrawQueryRepository,
    command: DynWithdrawCommandRepository,
    card_query: DynCardQueryRepository,
    saldo_query: DynSaldoQueryRepository,
    saldo_command: DynSaldoCommandRepository,
}

impl WithdrawCommandService {
    pub async fn new(
        query: DynWithdrawQueryRepository,
        command: DynWithdrawCommandRepository,
        card_query: DynCardQueryRepository,
        saldo_query: DynSaldoQueryRepository,
        saldo_command: DynSaldoCommandRepository,
    ) -> Self {
        Self {
            query,
            command,
            card_query,
            saldo_query,
            saldo_command,
        }
    }
}

#[async_trait]
impl WithdrawCommandServiceTrait for WithdrawCommandService {
    async fn create(
        &self,
        req: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("creating new withdraw: {:?}", req);

        let _card = self
            .card_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to find card".into())
            })?;

        let saldo = self
            .saldo_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom(format!("failed to find saldo for card {}", req.card_number))
            })?;

        if saldo.total_balance < req.withdraw_amount {
            error!(
                "error insufficient balance, requested: {}, available: {}",
                req.withdraw_amount, saldo.total_balance
            );
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        let new_total_balance = saldo.total_balance - req.withdraw_amount;

        let update_data = UpdateSaldoWithdraw {
            card_number: req.card_number.clone(),
            total_balance: new_total_balance as i32,
            withdraw_amount: req.withdraw_amount as i32,
            withdraw_time: req.withdraw_time,
        };

        self.saldo_command
            .update_withdraw(&update_data)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to update saldo".into())
            })?;

        let withdraw_record = match self.command.create(req).await {
            Ok(record) => {
                info!("created withdraw record {:?}", record.withdraw_id);
                record
            }
            Err(e) => {
                error!("error {e:?}");

                let rollback_data = UpdateSaldoWithdraw {
                    card_number: req.card_number.clone(),
                    total_balance: saldo.total_balance as i32,
                    withdraw_amount: req.withdraw_amount as i32,
                    withdraw_time: req.withdraw_time,
                };

                if let Err(rollback_err) = self.saldo_command.update_withdraw(&rollback_data).await
                {
                    error!("error rollback {rollback_err:?}");
                }

                return Err(ServiceError::Custom(
                    "failed to create withdraw record".into(),
                ));
            }
        };

        let update_status = UpdateWithdrawStatus {
            withdraw_id: withdraw_record.withdraw_id,
            status: "success".to_string(),
        };

        if let Err(e) = self.command.update_status(&update_status).await {
            error!("error {e:?}");

            let update_status_failed = UpdateWithdrawStatus {
                withdraw_id: withdraw_record.withdraw_id,
                status: "failed".to_string(),
            };

            if let Err(e2) = self.command.update_status(&update_status_failed).await {
                error!("error {e2:?}");
            }

            return Err(ServiceError::Custom(
                "failed to update withdraw status".into(),
            ));
        }

        info!("success withdraw {:?}", withdraw_record.withdraw_id);

        let withdraw_response = WithdrawResponse::from(withdraw_record);

        Ok(ApiResponse {
            status: "success".into(),
            message: "created withdraw successfully".into(),
            data: withdraw_response,
        })
    }

    async fn update(
        &self,
        req: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        info!("updating withdraw: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let _card = self
            .card_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to find card".into())
            })?;

        let _ = self.query.find_by_id(req.withdraw_id).await.map_err(|e| {
            error!("error {e:?}");
            ServiceError::Custom(format!("failed to find withdraw {}", req.withdraw_id))
        })?;

        let saldo = self
            .saldo_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom(format!(
                    "failed to fetch saldo for card {}",
                    req.card_number
                ))
            })?;

        if saldo.total_balance < req.withdraw_amount {
            error!(
                "error insufficient balance, requested: {}, available: {}",
                req.withdraw_amount, saldo.total_balance
            );
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        let new_total_balance = saldo.total_balance - req.withdraw_amount;

        let update_saldo_data = UpdateSaldoWithdraw {
            card_number: req.card_number.clone(),
            total_balance: new_total_balance as i32,
            withdraw_amount: req.withdraw_amount as i32,
            withdraw_time: req.withdraw_time,
        };

        if let Err(e) = self.saldo_command.update_withdraw(&update_saldo_data).await {
            error!("error {e:?}");

            if let Err(e2) = self
                .command
                .update_status(&UpdateWithdrawStatus {
                    withdraw_id: req.withdraw_id,
                    status: "failed".to_string(),
                })
                .await
            {
                error!("error {e2:?}");
            }

            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        let updated_withdraw = match self.command.update(req).await {
            Ok(record) => record,
            Err(e) => {
                error!("error {e:?}");

                let rollback_data = UpdateSaldoWithdraw {
                    card_number: req.card_number.clone(),
                    total_balance: saldo.total_balance as i32,
                    withdraw_amount: req.withdraw_amount as i32,
                    withdraw_time: req.withdraw_time,
                };
                if let Err(rollback_err) = self.saldo_command.update_withdraw(&rollback_data).await
                {
                    error!("error rollback {rollback_err:?}");
                }

                let _ = self
                    .command
                    .update_status(&UpdateWithdrawStatus {
                        withdraw_id: req.withdraw_id,
                        status: "failed".to_string(),
                    })
                    .await
                    .map_err(|e2| {
                        error!("error {e2:?}");
                        e2
                    });

                return Err(ServiceError::Custom(
                    "failed to update withdraw record".into(),
                ));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateWithdrawStatus {
                withdraw_id: updated_withdraw.withdraw_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("error {e:?}");

            let _ = self
                .command
                .update_status(&UpdateWithdrawStatus {
                    withdraw_id: updated_withdraw.withdraw_id,
                    status: "failed".to_string(),
                })
                .await
                .map_err(|e2| {
                    error!("error {e2:?}");
                    e2
                });

            return Err(ServiceError::Custom(
                "failed to update withdraw status".into(),
            ));
        }

        info!("success withdraw {:?}", updated_withdraw.withdraw_id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "updated withdraw successfully".into(),
            data: WithdrawResponse::from(updated_withdraw),
        })
    }

    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError> {
        info!("Trashing withdraw id={withdraw_id}");

        match self.command.trashed(withdraw_id).await {
            Ok(withdraw) => {
                let response = WithdrawResponseDeleteAt::from(withdraw);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Withdraw trashed successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("Failed to trash withdraw id={withdraw_id} {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to trash withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError> {
        info!("Restoring withdraw id={withdraw_id}");

        match self.command.restore(withdraw_id).await {
            Ok(withdraw) => {
                let response = WithdrawResponseDeleteAt::from(withdraw);
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Withdraw restored successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("Failed to restore withdraw id={withdraw_id} {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to restore withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("Permanently deleting withdraw id={withdraw_id}");

        match self.command.delete_permanent(withdraw_id).await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "Withdraw permanently deleted".into(),
                data: true,
            }),
            Err(e) => {
                error!("Failed to permanently delete withdraw id={withdraw_id} {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("Restoring all trashed withdraws");

        match self.command.restore_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "All withdraws restored successfully".into(),
                data: true,
            }),
            Err(e) => {
                error!("Failed to restore all withdraws: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to restore all trashed withdraws".to_string(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("Permanently deleting all trashed withdraws");

        match self.command.delete_all().await {
            Ok(_) => Ok(ApiResponse {
                status: "success".into(),
                message: "All withdraws permanently deleted".into(),
                data: true,
            }),
            Err(e) => {
                error!("Failed to permanently delete all withdraws: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed withdraws".to_string(),
                ))
            }
        }
    }
}
