use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        transfer::{
            repository::{
                command::DynTransferCommandRepository, query::DynTransferQueryRepository,
            },
            service::command::TransferCommandServiceTrait,
        },
    },
    domain::requests::{
        saldo::UpdateSaldoBalance,
        transfer::{CreateTransferRequest, UpdateTransferRequest, UpdateTransferStatus},
    },
    domain::responses::{ApiResponse, TransferResponse, TransferResponseDeleteAt},
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransferCommandService {
    card_query: DynCardQueryRepository,
    saldo_query: DynSaldoQueryRepository,
    saldo_command: DynSaldoCommandRepository,
    query: DynTransferQueryRepository,
    command: DynTransferCommandRepository,
}

impl TransferCommandService {
    pub async fn new(
        card_query: DynCardQueryRepository,
        saldo_query: DynSaldoQueryRepository,
        saldo_command: DynSaldoCommandRepository,
        query: DynTransferQueryRepository,
        command: DynTransferCommandRepository,
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
impl TransferCommandServiceTrait for TransferCommandService {
    async fn create(
        &self,
        req: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError> {
        info!("starting create transaction: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        if let Err(e) = self.card_query.find_by_card(&req.transfer_from).await {
            error!("error {e:?}");
            return Err(ServiceError::Custom(format!(
                "sender card {} not found",
                req.transfer_from
            )));
        }

        if let Err(e) = self.card_query.find_by_card(&req.transfer_to).await {
            error!("error {e:?}");
            return Err(ServiceError::Custom(format!(
                "receiver card {} not found",
                req.transfer_to
            )));
        }

        let mut sender_saldo = self
            .saldo_query
            .find_by_card(&req.transfer_from)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to fetch sender saldo".into())
            })?;

        let mut receiver_saldo = self
            .saldo_query
            .find_by_card(&req.transfer_to)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to fetch receiver saldo".into())
            })?;

        if sender_saldo.total_balance < req.transfer_amount {
            error!(
                "error insufficient balance, requested: {}, available: {}",
                req.transfer_amount, sender_saldo.total_balance
            );
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        sender_saldo.total_balance -= req.transfer_amount;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: sender_saldo.card_number.clone(),
                total_balance: sender_saldo.total_balance,
            })
            .await
        {
            error!("error {e:?}");
            return Err(ServiceError::Custom("failed to update sender saldo".into()));
        }

        receiver_saldo.total_balance += req.transfer_amount;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: receiver_saldo.card_number.clone(),
                total_balance: receiver_saldo.total_balance,
            })
            .await
        {
            error!("error {e:?}");

            sender_saldo.total_balance += req.transfer_amount;
            if let Err(rb) = self
                .saldo_command
                .update_balance(&UpdateSaldoBalance {
                    card_number: sender_saldo.card_number.clone(),
                    total_balance: sender_saldo.total_balance,
                })
                .await
            {
                error!("error rollback {rb:?}");
            }

            return Err(ServiceError::Custom(
                "failed to update receiver saldo".into(),
            ));
        }

        let transfer_record = match self.command.create(req).await {
            Ok(t) => t,
            Err(e) => {
                error!("error {e:?}");

                sender_saldo.total_balance += req.transfer_amount;
                receiver_saldo.total_balance -= req.transfer_amount;

                if let Err(rb1) = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: sender_saldo.card_number.clone(),
                        total_balance: sender_saldo.total_balance,
                    })
                    .await
                {
                    error!("error rollback sender {rb1:?}");
                }

                if let Err(rb2) = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: receiver_saldo.card_number.clone(),
                        total_balance: receiver_saldo.total_balance,
                    })
                    .await
                {
                    error!("error rollback receiver {rb2:?}");
                }

                return Err(ServiceError::Custom("failed to create transfer".into()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTransferStatus {
                transfer_id: transfer_record.transfer_id,
                status: "success".into(),
            })
            .await
        {
            error!("error {e:?}");
            return Err(ServiceError::Custom(
                "failed to update transfer status".into(),
            ));
        }

        info!(
            "successfully created transaction {:?}",
            transfer_record.transfer_id
        );

        let response = TransferResponse::from(transfer_record);
        Ok(ApiResponse {
            status: "success".into(),
            message: "transfer created successfully".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError> {
        info!("Starting update transaction process: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let transfer_id = req
            .transfer_id
            .ok_or_else(|| ServiceError::Custom("transfer_id is required".into()))?;

        let transfer = self.query.find_by_id(transfer_id).await.map_err(|e| {
            error!("error {e:?}");
            ServiceError::Custom(format!("failed to find transfer {transfer_id}"))
        })?;

        let amount_difference = req.transfer_amount - transfer.transfer_amount as i64;

        let mut sender_saldo = self
            .saldo_query
            .find_by_card(&transfer.transfer_from)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to fetch sender saldo".into())
            })?;

        let new_sender_balance = sender_saldo.total_balance - amount_difference;
        if new_sender_balance < 0 {
            error!("insufficient balance for sender {}", transfer.transfer_from);

            let _ = self
                .command
                .update_status(&UpdateTransferStatus {
                    transfer_id: transfer_id,
                    status: "failed".to_string(),
                })
                .await;

            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        sender_saldo.total_balance = new_sender_balance;
        self.saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: sender_saldo.card_number.clone(),
                total_balance: sender_saldo.total_balance,
            })
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to update sender saldo".into())
            })?;

        let mut receiver_saldo = match self.saldo_query.find_by_card(&transfer.transfer_to).await {
            Ok(s) => s,
            Err(e) => {
                error!("error {e:?}");

                let _ = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: sender_saldo.card_number.clone(),
                        total_balance: sender_saldo.total_balance + amount_difference,
                    })
                    .await;

                let _ = self
                    .command
                    .update_status(&UpdateTransferStatus {
                        transfer_id: transfer_id,
                        status: "failed".to_string(),
                    })
                    .await;

                return Err(ServiceError::Custom(
                    "failed to fetch receiver saldo".into(),
                ));
            }
        };

        receiver_saldo.total_balance += amount_difference;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: receiver_saldo.card_number.clone(),
                total_balance: receiver_saldo.total_balance,
            })
            .await
        {
            error!("error {e:?}");

            let _ = self
                .saldo_command
                .update_balance(&UpdateSaldoBalance {
                    card_number: sender_saldo.card_number.clone(),
                    total_balance: sender_saldo.total_balance + amount_difference,
                })
                .await;

            let _ = self
                .saldo_command
                .update_balance(&UpdateSaldoBalance {
                    card_number: receiver_saldo.card_number.clone(),
                    total_balance: receiver_saldo.total_balance - amount_difference,
                })
                .await;

            let _ = self
                .command
                .update_status(&UpdateTransferStatus {
                    transfer_id: transfer_id,
                    status: "failed".to_string(),
                })
                .await;

            return Err(ServiceError::Custom(
                "failed to update receiver saldo".into(),
            ));
        }

        let updated_transfer = match self.command.update(req).await {
            Ok(t) => t,
            Err(e) => {
                error!("error {e:?}");

                let _ = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: sender_saldo.card_number.clone(),
                        total_balance: sender_saldo.total_balance + amount_difference,
                    })
                    .await;

                let _ = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: receiver_saldo.card_number.clone(),
                        total_balance: receiver_saldo.total_balance - amount_difference,
                    })
                    .await;

                let _ = self
                    .command
                    .update_status(&UpdateTransferStatus {
                        transfer_id: transfer_id,
                        status: "failed".to_string(),
                    })
                    .await;

                return Err(ServiceError::Custom("failed to update transfer".into()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTransferStatus {
                transfer_id: transfer_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("error {e:?}");
            return Err(ServiceError::Custom(
                "failed to update transfer status".into(),
            ));
        }

        info!("successfully update transaction: {transfer_id}");

        Ok(ApiResponse {
            data: TransferResponse::from(updated_transfer),
            message: "Transfer updated successfully".into(),
            status: "success".into(),
        })
    }

    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing transfer id={transfer_id}");

        match self.command.trashed(transfer_id).await {
            Ok(transfer) => {
                info!("✅ Transfer trashed successfully: id={transfer_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transfer trashed successfully".into(),
                    data: TransferResponseDeleteAt::from(transfer),
                })
            }
            Err(e) => {
                error!("💥 Failed to trash transfer id={transfer_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to trash transfer with id {transfer_id}",
                )))
            }
        }
    }

    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring transfer id={transfer_id}");

        match self.command.restore(transfer_id).await {
            Ok(transfer) => {
                info!("✅ Transfer restored successfully: id={transfer_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transfer restored successfully".into(),
                    data: TransferResponseDeleteAt::from(transfer),
                })
            }
            Err(e) => {
                error!("💥 Failed to restore transfer id={transfer_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to restore transfer with id {transfer_id}",
                )))
            }
        }
    }

    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting transfer id={transfer_id}");

        match self.command.delete_permanent(transfer_id).await {
            Ok(_) => {
                info!("✅ Transfer permanently deleted: id={transfer_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transfer permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete transfer id={transfer_id}: {e:?}",);
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete transfer with id {transfer_id}",
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed transfers");

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All transfers restored successfully");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transfers restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all transfers: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to restore all trashed transfers".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed transfers");

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All transfers permanently deleted");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transfers permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete all transfers: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed transfers".into(),
                ))
            }
        }
    }
}
