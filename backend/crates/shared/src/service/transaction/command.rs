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
    cache::CacheStore,
    domain::requests::{
        saldo::UpdateSaldoBalance,
        transaction::{
            CreateTransactionRequest, UpdateTransactionRequest, UpdateTransactionStatus,
        },
    },
    domain::responses::{ApiResponse, TransactionResponse, TransactionResponseDeleteAt},
    errors::{ServiceError, format_validation_errors},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
};
use anyhow::Result;
use async_trait::async_trait;
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionCommandService {
    pub query: DynTransactionQueryRepository,
    pub command: DynTransactionCommandRepository,
    pub merchant_query: DynMerchantQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub card_query: DynCardQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct TransactionCommandServiceDeps {
    pub query: DynTransactionQueryRepository,
    pub command: DynTransactionCommandRepository,
    pub merchant_query: DynMerchantQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub card_query: DynCardQueryRepository,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionCommandService {
    pub fn new(deps: TransactionCommandServiceDeps) -> Result<Self> {
        let metrics = Metrics::new();

        let TransactionCommandServiceDeps {
            query,
            command,
            merchant_query,
            saldo_query,
            saldo_command,
            card_query,
            cache_store,
        } = deps;

        Ok(Self {
            query,
            command,
            merchant_query,
            saldo_query,
            saldo_command,
            card_query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transaction-command-service")
    }
    fn inject_trace_context<T>(&self, cx: &Context, request: &mut Request<T>) {
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(cx, &mut MetadataInjector(request.metadata_mut()))
        });
    }

    fn start_tracing(&self, operation_name: &str, attributes: Vec<KeyValue>) -> TracingContext {
        let start_time = Instant::now();
        let tracer = self.get_tracer();
        let mut span = tracer
            .span_builder(operation_name.to_string())
            .with_kind(SpanKind::Server)
            .with_attributes(attributes)
            .start(&tracer);

        info!("Starting operation: {operation_name}");

        span.add_event(
            "Operation started",
            vec![
                KeyValue::new("operation", operation_name.to_string()),
                KeyValue::new("timestamp", start_time.elapsed().as_secs_f64().to_string()),
            ],
        );

        let cx = Context::current_with_span(span);
        TracingContext { cx, start_time }
    }

    async fn complete_tracing_success(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, true, message)
            .await;
    }

    async fn complete_tracing_error(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        error_message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, false, error_message)
            .await;
    }

    async fn complete_tracing_internal(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        is_success: bool,
        message: &str,
    ) {
        let status_str = if is_success { "SUCCESS" } else { "ERROR" };
        let status = if is_success {
            StatusUtils::Success
        } else {
            StatusUtils::Error
        };
        let elapsed = tracing_ctx.start_time.elapsed().as_secs_f64();

        tracing_ctx.cx.span().add_event(
            "Operation completed",
            vec![
                KeyValue::new("status", status_str),
                KeyValue::new("duration_secs", elapsed.to_string()),
                KeyValue::new("message", message.to_string()),
            ],
        );

        if is_success {
            info!("✅ Operation completed successfully: {message}");
        } else {
            error!("❌ Operation failed: {message}");
        }

        self.metrics.record(method, status, elapsed);

        tracing_ctx.cx.span().end();
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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_transaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "create"),
                KeyValue::new("transaction.amount", req.amount.to_string()),
                KeyValue::new("transaction.card_number", req.card_number.clone()),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        let merchant = match self.merchant_query.find_by_apikey(api_key).await {
            Ok(merchant) => {
                info!("merchant found with api_key: {}", api_key);
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    "merchant retrieved successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                error!("error finding merchant with api_key {}: {:?}", api_key, e);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("failed to find merchant: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("failed to find merchant".into()));
            }
        };

        let card = match self.card_query.find_by_card(&req.card_number).await {
            Ok(card) => {
                info!("card found: {}", req.card_number);
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    "card retrieved successfully",
                )
                .await;
                card
            }
            Err(e) => {
                error!("error finding card {}: {:?}", req.card_number, e);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("failed to find card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("failed to find card".into()));
            }
        };

        let mut saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(saldo) => {
                info!("saldo found for card {}", req.card_number);
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    "saldo retrieved successfully",
                )
                .await;
                saldo
            }
            Err(e) => {
                error!("error finding saldo for card {}: {:?}", req.card_number, e);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("failed to fetch saldo: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("failed to fetch saldo".into()));
            }
        };

        if saldo.total_balance < req.amount {
            let error_msg = format!(
                "insufficient balance, requested: {}, available: {}",
                req.amount, saldo.total_balance
            );
            error!("{error_msg}");

            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
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

            let error_msg = "failed to update saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
        }

        let mut req_with_merchant = req.clone();
        req_with_merchant.merchant_id = Some(merchant.merchant_id);

        let transaction = match self.command.create(&req_with_merchant).await {
            Ok(tx) => tx,
            Err(e) => {
                let error_msg = format!("failed to create transaction {e:?}");
                error!("{error_msg}");

                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

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
            let error_msg = "failed to update transaction status";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
        }

        let merchant_card = match self.card_query.find_by_user_id(merchant.user_id).await {
            Ok(card) => card,
            Err(e) => {
                error!("error {e:?}");
                let error_msg = "failed to fetch merchant card";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;
                return Err(ServiceError::Custom(error_msg.into()));
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
                let error_msg = "failed to fetch merchant saldo";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;
                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        merchant_saldo.total_balance += req.amount;

        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: merchant_card.clone().card_number,
                total_balance: merchant_saldo.total_balance,
            })
            .await
        {
            error!("failed to update merchant saldo {e:?}");

            let error_msg = "failed to update merchant saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
        }

        let cache_keys = vec![
            format!("transaction:find_by_card:{}", req.card_number),
            format!("saldo:find_by_card:{}", req.card_number),
            format!("saldo:find_by_card:{}", merchant_card.card_number),
            "transaction:find_all:*".to_string(),
            "transaction:find_by_active:*".to_string(),
            "transaction:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }

        let response = TransactionResponse::from(transaction);

        info!(
            "CreateTransaction completed, api_key: {api_key}, transaction_id: {}",
            response.id
        );

        self.complete_tracing_success(&tracing_ctx, method, "Transaction created successfully")
            .await;

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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_transaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "update"),
                KeyValue::new(
                    "transaction.id",
                    req.transaction_id.unwrap_or_default().to_string(),
                ),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());

        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        let transaction_id = match req.transaction_id {
            Some(id) => id,
            None => {
                let msg = "transaction_id is required";
                self.complete_tracing_error(&tracing_ctx, method.clone(), msg)
                    .await;
                return Err(ServiceError::Custom(msg.into()));
            }
        };

        let mut transaction = match self.query.find_by_id(transaction_id).await {
            Ok(v) => v,
            Err(e) => {
                let msg = format!("transaction {} not found", transaction_id);
                error!("failed to find transaction: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let merchant = match self.merchant_query.find_by_apikey(api_key).await {
            Ok(v) => v,
            Err(e) => {
                let msg = "failed to fetch merchant";
                error!("{msg}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", msg, e),
                )
                .await;
                return Err(ServiceError::Custom(msg.into()));
            }
        };

        if transaction.clone().merchant_id != merchant.merchant_id {
            error!("unauthorized access to transaction {}", transaction_id);

            let _ = self
                .command
                .update_status(&UpdateTransactionStatus {
                    transaction_id,
                    status: "failed".into(),
                })
                .await;

            let error_msg = "unauthorized access";
            self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                .await;

            return Err(ServiceError::Custom(error_msg.into()));
        }

        let card = match self.card_query.find_by_card(&transaction.card_number).await {
            Ok(v) => v,
            Err(e) => {
                error!("failed to find card: {e:?}");

                let error_msg = "card not found";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let mut saldo = match self.saldo_query.find_by_card(&card.card_number).await {
            Ok(v) => v,
            Err(e) => {
                error!("failed to find saldo: {e:?}");

                let error_msg = "saldo not found";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        saldo.total_balance += transaction.amount as i64;
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
                    transaction_id,
                    status: "failed".into(),
                })
                .await;

            let error_msg = "failed to restore saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;

            return Err(ServiceError::Custom(error_msg.into()));
        }

        if saldo.total_balance < req.amount {
            let error_msg = format!(
                "insufficient balance, available: {}, requested: {}",
                saldo.total_balance, req.amount
            );
            error!("{error_msg}");
            let _ = self
                .command
                .update_status(&UpdateTransactionStatus {
                    transaction_id,
                    status: "failed".into(),
                })
                .await;

            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;

            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        saldo.total_balance -= req.amount;
        let _ = match self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: card.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
        {
            Ok(v) => v,

            Err(e) => {
                error!("failed to update saldo: {e:?}");

                let error_msg = "failed to update saldo";

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        transaction.amount = req.amount as i32;
        transaction.payment_method = req.payment_method.clone();

        let updated = match self
            .command
            .update(&UpdateTransactionRequest {
                transaction_id: Some(transaction_id),
                card_number: transaction.card_number.clone(),
                amount: transaction.amount as i64,
                payment_method: transaction.payment_method.clone(),
                merchant_id: Some(transaction.merchant_id),
                transaction_time: transaction.transaction_time,
            })
            .await
        {
            Ok(v) => v,
            Err(e) => {
                error!("failed to update transaction: {e:?}");

                let error_msg = "failed to update transaction";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let _ = match self
            .command
            .update_status(&UpdateTransactionStatus {
                transaction_id,
                status: "success".into(),
            })
            .await
        {
            Ok(v) => v,
            Err(e) => {
                error!("failed to update transaction status: {e:?}");

                let error_msg = "failed to update transaction status";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let cache_keys = vec![
            format!("transaction:find_by_id:{}", transaction_id),
            format!("transaction:find_by_card:{}", transaction.card_number),
            format!("saldo:find_by_card:{}", transaction.card_number),
            format!("transaction:find_all:*"),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }

        let response = TransactionResponse::from(updated);

        self.complete_tracing_success(&tracing_ctx, method, "Transaction updated successfully")
            .await;

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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_transaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut request = Request::new(transaction_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let transaction = match self.command.trashed(transaction_id).await {
            Ok(transaction) => {
                info!(
                    "✅ Transaction trashed successfully with id={}",
                    transaction.transaction_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transaction trashed successfully",
                )
                .await;
                transaction
            }
            Err(e) => {
                error!("💥 Failed to trash transaction id={transaction_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash transaction: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to trash transaction with id {}",
                    transaction_id
                )));
            }
        };

        let response = TransactionResponseDeleteAt::from(transaction);

        let cache_keys = vec![
            format!("transaction:find_by_id:id:{}", transaction_id),
            format!("card:find_by_card:number:{}", response.card_number),
            "transaction:find_all:*".to_string(),
            "transaction:find_active:*".to_string(),
            "transaction:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🗑️ Transaction trashed successfully!".into(),
            data: response,
        })
    }

    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring transaction id={transaction_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_transaction",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut request = Request::new(transaction_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let transaction = match self.command.restore(transaction_id).await {
            Ok(transaction) => {
                info!(
                    "✅ Transaction restored successfully with id={}",
                    transaction.transaction_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transaction restored successfully",
                )
                .await;
                transaction
            }
            Err(e) => {
                error!("💥 Failed to restore transaction id={transaction_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore transaction: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to restore transaction with id {}",
                    transaction_id
                )));
            }
        };

        let response = TransactionResponseDeleteAt::from(transaction);

        let cache_keys = vec![
            format!("transaction:find_by_id:id:{}", transaction_id),
            format!("card:find_by_card:number:{}", response.card_number),
            "transaction:find_all:*".to_string(),
            "transaction:find_active:*".to_string(),
            "transaction:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "♻️ Transaction restored successfully!".into(),
            data: response,
        })
    }

    async fn delete_permanent(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting transaction id={transaction_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_permanent",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut request = Request::new(transaction_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(transaction_id).await {
            Ok(_) => {
                info!("✅ Transaction permanently deleted: id={transaction_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transaction permanently deleted successfully",
                )
                .await;
            }
            Err(e) => {
                error!("💥 Failed to permanently delete transaction id={transaction_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete transaction: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to permanently delete transaction with id {}",
                    transaction_id
                )));
            }
        }

        let cache_keys = vec![
            format!("transaction:find_by_id:id:{}", transaction_id),
            "transaction:find_all:*".to_string(),
            "transaction:find_active:*".to_string(),
            "transaction:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🧨 Transaction permanently deleted!".into(),
            data: true,
        })
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed transactions");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_transactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All transactions restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All transactions restored successfully",
                )
                .await;
            }
            Err(e) => {
                error!("💥 Failed to restore all transactions: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(
                    "Failed to restore all trashed transactions".into(),
                ));
            }
        }

        let cache_keys = vec![
            "transaction:find_all:*".to_string(),
            "transaction:find_active:*".to_string(),
            "transaction:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🔄 All trashed transactions restored successfully!".into(),
            data: true,
        })
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed transactions");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_transactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All transactions permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All transactions permanently deleted successfully",
                )
                .await;
            }
            Err(e) => {
                error!("💥 Failed to delete all transactions: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to delete all transactions: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(
                    "Failed to delete all trashed transactions".into(),
                ));
            }
        }

        let cache_keys = vec![
            "transaction:find_all:*".to_string(),
            "transaction:find_active:*".to_string(),
            "transaction:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "💣 All trashed transactions permanently deleted!".into(),
            data: true,
        })
    }
}
