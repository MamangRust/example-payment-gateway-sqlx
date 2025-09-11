use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        merchant::repository::query::DynMerchantQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        transaction::{
            repository::{
                command::DynTransactionCommandRepository, query::DynTransactionQueryRepository,
            },
            service::command::TransactionCommandServiceTrait,
        },
    },
    domain::requests::{
        saldo::UpdateSaldoBalance,
        transaction::{
            CreateTransactionRequest, UpdateTransactionRequest, UpdateTransactionStatus,
        },
    },
    domain::responses::{ApiResponse, TransactionResponse, TransactionResponseDeleteAt},
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionCommandService {
    query: DynTransactionQueryRepository,
    command: DynTransactionCommandRepository,
    merchant_query: DynMerchantQueryRepository,
    saldo_query: DynSaldoQueryRepository,
    saldo_command: DynSaldoCommandRepository,
    card_query: DynCardQueryRepository,
}

impl TransactionCommandService {
    pub async fn new(
        query: DynTransactionQueryRepository,
        command: DynTransactionCommandRepository,
        merchant_query: DynMerchantQueryRepository,
        saldo_query: DynSaldoQueryRepository,
        saldo_command: DynSaldoCommandRepository,
        card_query: DynCardQueryRepository,
    ) -> Self {
        Self {
            query,
            command,
            merchant_query,
            saldo_query,
            saldo_command,
            card_query,
        }
    }
}

#[async_trait]
impl TransactionCommandServiceTrait for TransactionCommandService {
    async fn create(
        &self,
        api_key: &str,
        req: &CreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError> {
        info!(
            "starting CreateTransaction process, api_key: {api_key}, req: {:?}",
            req
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let merchant = self
            .merchant_query
            .find_by_apikey(&api_key)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to find merchant".into())
            })?;

        let card = self
            .card_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to find card".into())
            })?;

        let mut saldo = self
            .saldo_query
            .find_by_card(&req.card_number)
            .await
            .map_err(|e| {
                error!("error {e:?}");
                ServiceError::Custom("failed to fetch saldo".into())
            })?;

        if saldo.total_balance < req.amount {
            error!(
                "insufficient balance, requested: {}, available: {}",
                req.amount, saldo.total_balance
            );
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        saldo.total_balance -= req.amount;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: card.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
        {
            error!("failed to update saldo {e:?}");
            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        let mut req_with_merchant = req.clone();

        req_with_merchant.merchant_id = Some(merchant.merchant_id);

        let transaction = match self.command.create(&req_with_merchant).await {
            Ok(tx) => tx,
            Err(e) => {
                error!("failed to create transaction {e:?}");

                saldo.total_balance += req.amount;
                if let Err(rollback_err) = self
                    .saldo_command
                    .update_balance(&UpdateSaldoBalance {
                        card_number: card.card_number.clone(),
                        total_balance: saldo.total_balance,
                    })
                    .await
                {
                    error!("failed to rollback saldo {rollback_err:?}");
                }

                return Err(ServiceError::Custom("failed to create transaction".into()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTransactionStatus {
                transaction_id: transaction.transaction_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("failed to update transaction status {e:?}");
            return Err(ServiceError::Custom(
                "failed to update transaction status".into(),
            ));
        }

        let merchant_card = match self.card_query.find_by_user_id(merchant.user_id).await {
            Ok(card) => card,
            Err(e) => {
                error!("error {e:?}");
                return Err(ServiceError::Custom("failed to fetch merchant card".into()));
            }
        };

        let mut merchant_saldo = match self
            .saldo_query
            .find_by_card(&merchant_card.card_number)
            .await
        {
            Ok(saldo) => saldo,
            Err(e) => {
                error!("error {e:?}");
                return Err(ServiceError::Custom(
                    "failed to fetch merchant saldo".into(),
                ));
            }
        };

        merchant_saldo.total_balance += req.amount;

        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: merchant_card.card_number,
                total_balance: merchant_saldo.total_balance,
            })
            .await
        {
            error!("failed to update merchant saldo {e:?}");
            return Err(ServiceError::Custom(
                "failed to update merchant saldo".into(),
            ));
        }

        let response = TransactionResponse::from(transaction);

        info!(
            "CreateTransaction completed, api_key: {api_key}, transaction_id: {}",
            response.id
        );

        Ok(ApiResponse {
            status: "success".into(),
            message: "transaction created successfully".into(),
            data: response,
        })
    }
    async fn update(
        &self,
        api_key: &str,
        req: &UpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError> {
        info!("Starting UpdateTransaction process: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let mut transaction = self
            .query
            .find_by_id(req.transaction_id)
            .await
            .map_err(|e| {
                error!("failed to find transaction: {e:?}");
                ServiceError::Custom(format!("transaction {} not found", req.transaction_id))
            })?;

        let merchant = self
            .merchant_query
            .find_by_apikey(api_key)
            .await
            .map_err(|e| {
                error!("failed to find merchant: {e:?}");
                ServiceError::Custom("failed to fetch merchant".into())
            })?;

        if transaction.clone().merchant_id != merchant.merchant_id {
            error!("unauthorized access to transaction {}", req.transaction_id);

            let _ = self
                .command
                .update_status(&UpdateTransactionStatus {
                    transaction_id: req.transaction_id,
                    status: "failed".into(),
                })
                .await;
            return Err(ServiceError::Custom("unauthorized access".into()));
        }

        let card = self
            .card_query
            .find_by_card(&transaction.card_number)
            .await
            .map_err(|e| {
                error!("failed to find card: {e:?}");
                ServiceError::Custom("card not found".into())
            })?;

        let mut saldo = self
            .saldo_query
            .find_by_card(&card.card_number)
            .await
            .map_err(|e| {
                error!("failed to find saldo: {e:?}");
                ServiceError::Custom("saldo not found".into())
            })?;

        saldo.total_balance += transaction.amount;
        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: card.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
        {
            error!("failed to restore balance: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTransactionStatus {
                    transaction_id: req.transaction_id,
                    status: "failed".into(),
                })
                .await;
            return Err(ServiceError::Custom("failed to restore saldo".into()));
        }

        if saldo.total_balance < req.amount {
            error!(
                "insufficient balance, available: {}, requested: {}",
                saldo.total_balance, req.amount
            );
            let _ = self
                .command
                .update_status(&UpdateTransactionStatus {
                    transaction_id: req.transaction_id,
                    status: "failed".into(),
                })
                .await;
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        saldo.total_balance -= req.amount;
        self.saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: card.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
            .map_err(|e| {
                error!("failed to update saldo: {e:?}");
                ServiceError::Custom("failed to update saldo".into())
            })?;

        transaction.amount = req.amount;
        transaction.payment_method = req.payment_method.clone();

        let updated = self
            .command
            .update(&UpdateTransactionRequest {
                transaction_id: req.transaction_id,
                card_number: transaction.card_number.clone(),
                amount: transaction.amount,
                payment_method: transaction.payment_method.clone(),
                merchant_id: Some(transaction.merchant_id),
                transaction_time: transaction.transaction_time,
            })
            .await
            .map_err(|e| {
                error!("failed to update transaction: {e:?}");
                ServiceError::Custom("failed to update transaction".into())
            })?;

        self.command
            .update_status(&UpdateTransactionStatus {
                transaction_id: req.transaction_id,
                status: "success".into(),
            })
            .await
            .map_err(|e| {
                error!("failed to update transaction status: {e:?}");
                ServiceError::Custom("failed to update transaction status".into())
            })?;

        let response = TransactionResponse::from(updated);
        Ok(ApiResponse {
            message: "Transaction updated successfully".into(),
            status: "success".into(),
            data: response,
        })
    }
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing transaction id={transaction_id}");

        match self.command.trashed(transaction_id).await {
            Ok(transaction) => {
                info!("✅ Transaction trashed successfully: id={transaction_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transaction trashed successfully".into(),
                    data: TransactionResponseDeleteAt::from(transaction),
                })
            }
            Err(e) => {
                error!("💥 Failed to trash transaction id={transaction_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to trash transaction with id {transaction_id}"
                )))
            }
        }
    }

    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring transaction id={transaction_id}");

        match self.command.restore(transaction_id).await {
            Ok(transaction) => {
                info!("✅ Transaction restored successfully: id={transaction_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transaction restored successfully".into(),
                    data: TransactionResponseDeleteAt::from(transaction),
                })
            }
            Err(e) => {
                error!("💥 Failed to restore transaction id={transaction_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to restore transaction with id {transaction_id}"
                )))
            }
        }
    }

    async fn delete_permanent(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting transaction id={transaction_id}");

        match self.command.delete_permanent(transaction_id).await {
            Ok(_) => {
                info!("✅ Transaction permanently deleted: id={transaction_id}");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transaction permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete transaction id={transaction_id}: {e:?}");
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete transaction with id {transaction_id}"
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring all trashed transactions");

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All transactions restored successfully");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transactions restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all transactions: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to restore all trashed transactions".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting all trashed transactions");

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All transactions permanently deleted");
                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transactions permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete all transactions: {e:?}");
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed transactions".into(),
                ))
            }
        }
    }
}
