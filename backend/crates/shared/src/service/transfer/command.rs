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
    cache::CacheStore,
    domain::requests::{
        saldo::UpdateSaldoBalance,
        transfer::{CreateTransferRequest, UpdateTransferRequest, UpdateTransferStatus},
    },
    domain::responses::{ApiResponse, TransferResponse, TransferResponseDeleteAt},
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

pub struct TransferCommandService {
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub query: DynTransferQueryRepository,
    pub command: DynTransferCommandRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct TransferCommandServiceDeps {
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub query: DynTransferQueryRepository,
    pub command: DynTransferCommandRepository,
    pub cache_store: Arc<CacheStore>,
}

impl TransferCommandService {
    pub fn new(deps: TransferCommandServiceDeps) -> Result<Self> {
        let metrics = Metrics::new();

        let TransferCommandServiceDeps {
            card_query,
            saldo_query,
            saldo_command,
            query,
            command,
            cache_store,
        } = deps;

        Ok(Self {
            card_query,
            saldo_query,
            saldo_command,
            query,
            command,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("transfer-command-service")
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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_transfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "create"),
                KeyValue::new("transfer.amount", req.transfer_amount.to_string()),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        if let Err(e) = self.card_query.find_by_card(&req.transfer_from).await {
            error!("error {e:?}");
            let error_msg = format!("sender card {} not found", req.transfer_from);
            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
            return Err(ServiceError::Custom(error_msg));
        }

        if let Err(e) = self.card_query.find_by_card(&req.transfer_to).await {
            error!("error {e:?}");
            let error_msg = format!("receiver card {} not found", req.transfer_to);
            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
            return Err(ServiceError::Custom(error_msg));
        }

        let mut sender_saldo = match self.saldo_query.find_by_card(&req.transfer_from).await {
            Ok(saldo) => saldo,
            Err(e) => {
                let error_msg = "failed to fetch sender saldo";
                error!("{error_msg}: {e:?}");

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let mut receiver_saldo = match self.saldo_query.find_by_card(&req.transfer_to).await {
            Ok(saldo) => saldo,
            Err(e) => {
                let error_msg = "failed to fetch receiver saldo";
                error!("{error_msg}: {e:?}");

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        if sender_saldo.total_balance < req.transfer_amount {
            let error_msg = format!(
                "error insufficient balance, requested: {}, available: {}",
                req.transfer_amount, sender_saldo.total_balance
            );
            error!("{error_msg}");

            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
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

            let error_msg = "failed to update sender saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
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
            let error_msg = "failed to update receiver saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;

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

            return Err(ServiceError::Custom(error_msg.into()));
        }

        let transfer_record = match self.command.create(req).await {
            Ok(t) => t,
            Err(e) => {
                let error_msg = format!("failed to create transfer: {:?}", e);
                error!("{error_msg}");

                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

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
            let error_msg = "failed to update transfer status";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
        }

        let cache_keys = vec![
            format!("saldo:find_by_card:{}", req.transfer_from),
            format!("saldo:find_by_card:{}", req.transfer_to),
            "transfer:find_all:*".to_string(),
            "transfer:find_by_active:*".to_string(),
            "transfer:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }

        info!(
            "successfully created transaction {:?}",
            transfer_record.transfer_id
        );

        let response = TransferResponse::from(transfer_record);

        self.complete_tracing_success(&tracing_ctx, method, "Transfer created successfully")
            .await;

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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_transfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "update"),
                KeyValue::new(
                    "transfer.id",
                    req.transfer_id.unwrap_or_default().to_string(),
                ),
                KeyValue::new("transfer.amount", req.transfer_amount.to_string()),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        let transfer_id = match req.transfer_id {
            Some(id) => id,
            None => {
                let error_msg = "transfer_id is required";
                self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let transfer = match self.query.find_by_id(transfer_id).await {
            Ok(data) => data,
            Err(e) => {
                let error_msg = format!("failed to find transfer {}", transfer_id);
                error!("{error_msg}: {e:?}");

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.to_string()));
            }
        };

        let amount_difference = req.transfer_amount - transfer.transfer_amount as i64;

        let mut sender_saldo = match self.saldo_query.find_by_card(&transfer.transfer_from).await {
            Ok(s) => s,
            Err(e) => {
                let error_msg = "failed to fetch sender saldo";
                error!("{error_msg}: {e:?}");

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let new_sender_balance = sender_saldo.total_balance - amount_difference;
        if new_sender_balance < 0 {
            let error_msg = format!("insufficient balance for sender {}", transfer.transfer_from);
            error!("{error_msg}");

            let _ = self
                .command
                .update_status(&UpdateTransferStatus {
                    transfer_id,
                    status: "failed".to_string(),
                })
                .await;

            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;

            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        sender_saldo.total_balance = new_sender_balance;

        match self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: sender_saldo.card_number.clone(),
                total_balance: sender_saldo.total_balance,
            })
            .await
        {
            Ok(_) => (),
            Err(e) => {
                let error_msg = "failed to update sender saldo";
                error!("{error_msg}: {e:?}");

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        }

        let mut receiver_saldo = match self.saldo_query.find_by_card(&transfer.transfer_to).await {
            Ok(s) => s,
            Err(e) => {
                error!("error {e:?}");
                let error_msg = "failed to fetch receiver saldo";
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

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
                        transfer_id,
                        status: "failed".to_string(),
                    })
                    .await;

                return Err(ServiceError::Custom(error_msg.into()));
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
            let error_msg = "failed to update receiver saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;

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
                    transfer_id,
                    status: "failed".to_string(),
                })
                .await;

            return Err(ServiceError::Custom(error_msg.into()));
        }

        let updated_transfer = match self.command.update(req).await {
            Ok(t) => t,
            Err(e) => {
                let error_msg = format!("failed to update transfer: {:?}", e);
                error!("{error_msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

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
                        transfer_id,
                        status: "failed".to_string(),
                    })
                    .await;

                return Err(ServiceError::Custom("failed to update transfer".into()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTransferStatus {
                transfer_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("error {e:?}");

            let error_msg = "failed to update transfer status";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;
            return Err(ServiceError::Custom(error_msg.into()));
        }

        let cache_keys = vec![
            format!("transfer:find_by_id:{}", transfer_id),
            format!("saldo:find_by_card:{}", transfer.transfer_from),
            format!("saldo:find_by_card:{}", transfer.transfer_to),
            "transfer:find_all:*".to_string(),
            "transfer:find_by_active:*".to_string(),
            "transfer:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }

        info!("successfully update transaction: {transfer_id}");

        self.complete_tracing_success(&tracing_ctx, method, "Transfer updated successfully")
            .await;

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

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_transfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("transfer_id", transfer_id.to_string()),
            ],
        );

        let mut request = Request::new(transfer_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let transfer = match self.command.trashed(transfer_id).await {
            Ok(transfer) => {
                info!("✅ Transfer trashed successfully: id={transfer_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transfer trashed successfully",
                )
                .await;

                let cache_keys = vec![
                    "transfer:find_all:*".to_string(),
                    "transfer:find_active:*".to_string(),
                    "transfer:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                transfer
            }
            Err(e) => {
                error!("💥 Failed to trash transfer id={transfer_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash transfer: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to trash transfer with id {transfer_id}",
                )));
            }
        };

        let cache_pattern = format!("transfer:find_by_id:id:{}*", transfer_id);

        self.cache_store.delete_from_cache(&cache_pattern).await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "Transfer trashed successfully".into(),
            data: TransferResponseDeleteAt::from(transfer),
        })
    }

    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring transfer id={transfer_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_transfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("transfer_id", transfer_id.to_string()),
            ],
        );

        let mut request = Request::new(transfer_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let transfer = match self.command.restore(transfer_id).await {
            Ok(transfer) => {
                info!("✅ Transfer restored successfully: id={transfer_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transfer restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "transfer:find_all:*".to_string(),
                    "transfer:find_active:*".to_string(),
                    "transfer:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                transfer
            }
            Err(e) => {
                error!("💥 Failed to restore transfer id={transfer_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore transfer: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to restore transfer with id {transfer_id}",
                )));
            }
        };

        let cache_pattern = format!("transfer:find_by_id:id:{}*", transfer_id);

        self.cache_store.delete_from_cache(&cache_pattern).await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "Transfer restored successfully".into(),
            data: TransferResponseDeleteAt::from(transfer),
        })
    }

    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting transfer id={transfer_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_permanent_transfer",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("transfer_id", transfer_id.to_string()),
            ],
        );

        let mut request = Request::new(transfer_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(transfer_id).await {
            Ok(_) => {
                info!("✅ Transfer permanently deleted: id={transfer_id}");
                self.complete_tracing_success(&tracing_ctx, method, "Transfer permanently deleted")
                    .await;

                let cache_keys = vec![
                    format!("transfer:find_by_id:id:{}", transfer_id),
                    "transfer:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Transfer permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete transfer id={transfer_id}: {e:?}",);
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete transfer: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete transfer with id {transfer_id}",
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed transfers");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_transfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All transfers restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All transfers restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "transfer:find_trashed:*",
                    "transfer:find_active:*",
                    "transfer:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transfers restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all transfers: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all transfers: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to restore all trashed transfers".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed transfers");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_transfers",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All transfers permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All transfers permanently deleted",
                )
                .await;

                let cache_keys = vec![
                    "transfer:find_trashed:*",
                    "transfer:find_active:*",
                    "transfer:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All transfers permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete all transfers: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete all transfers: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed transfers".into(),
                ))
            }
        }
    }
}
