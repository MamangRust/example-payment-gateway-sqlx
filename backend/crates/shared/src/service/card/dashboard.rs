use crate::{
    abstract_trait::card::{
        repository::dashboard::{
            balance::DynCardDashboardBalanceRepository, topup::DynCardDashboardTopupRepository,
            transaction::DynCardDashboardTransactionRepository,
            transfer::DynCardDashboardTransferRepository,
            withdraw::DynCardDashboardWithdrawRepository,
        },
        service::dashboard::CardDashboardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::ServiceError,
    observability::{Method, TracingMetrics},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};

pub struct CardDashboardService {
    pub balance: DynCardDashboardBalanceRepository,
    pub topup: DynCardDashboardTopupRepository,
    pub transaction: DynCardDashboardTransactionRepository,
    pub transfer: DynCardDashboardTransferRepository,
    pub withdraw: DynCardDashboardWithdrawRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct CardDashboardServiceDeps {
    pub balance: DynCardDashboardBalanceRepository,
    pub topup: DynCardDashboardTopupRepository,
    pub transaction: DynCardDashboardTransactionRepository,
    pub transfer: DynCardDashboardTransferRepository,
    pub withdraw: DynCardDashboardWithdrawRepository,
}

impl CardDashboardService {
    pub fn new(deps: CardDashboardServiceDeps, shared: &SharedResources) -> Result<Self> {
        let CardDashboardServiceDeps {
            balance,
            topup,
            transaction,
            transfer,
            withdraw,
        } = deps;

        Ok(Self {
            balance,
            topup,
            transaction,
            transfer,
            withdraw,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl CardDashboardServiceTrait for CardDashboardService {
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, ServiceError> {
        info!("📊 Fetching global dashboard statistics (strict mode)");

        let method = Method::Get;

        let tracing_ctx = self
            .tracing_metrics_core
            .start_tracing("get_dashboard", vec![]);

        let mut request = Request::new(());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = "dashboard:global".to_string();

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<DashboardCard>>(&cache_key)
            .await
        {
            info!("✅ Found global dashboard in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Global dashboard retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let total_balance = match self.balance.get_total_balance().await {
            Ok(balance) => balance,
            Err(e) => {
                error!("❌ Failed to get total balance: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to get total balance",
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_topup = match self.topup.get_total_amount().await {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get total top-up: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to get total top-up",
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_transaction = match self.transaction.get_total_amount().await {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get total transaction: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to get total transaction",
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_transfer = match self.transfer.get_total_amount().await {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get total transfer: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to get total transfer",
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_withdraw = match self.withdraw.get_total_amount().await {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get total withdraw: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to get total withdraw",
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let dashboard = DashboardCard {
            total_balance: Some(total_balance),
            total_topup: Some(total_topup),
            total_transaction: Some(total_transaction),
            total_transfer: Some(total_transfer),
            total_withdraw: Some(total_withdraw),
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Global dashboard retrieved successfully".to_string(),
            data: dashboard,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!("✅ Global dashboard retrieved successfully");
        self.tracing_metrics_core
            .complete_tracing_success(
                &tracing_ctx,
                method,
                "Global dashboard retrieved successfully",
            )
            .await;

        Ok(response)
    }

    async fn get_dashboard_bycard(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, ServiceError> {
        info!("💳📊 Fetching dashboard for card: {}", card_number);

        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_dashboard_bycard",
            vec![KeyValue::new("card_number", mask_card_number(&card_number))],
        );

        let mut request = Request::new(card_number.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("dashboard:card:{}", mask_card_number(&card_number));

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<DashboardCardCardNumber>>(&cache_key)
            .await
        {
            info!("✅ Found card dashboard in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Card dashboard retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let total_balance = match self
            .balance
            .get_total_balance_by_card(card_number.clone())
            .await
        {
            Ok(balance) => balance,
            Err(e) => {
                error!("❌ Failed to get balance for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get balance for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_topup = match self
            .topup
            .get_total_amount_by_card(card_number.clone())
            .await
        {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get top-up for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get top-up for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_transaction = match self
            .transaction
            .get_total_amount_by_card(card_number.clone())
            .await
        {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get transaction for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get transaction for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_transfer_send = match self
            .transfer
            .get_total_amount_by_sender(card_number.clone())
            .await
        {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get transfer (sent) for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get transfer (sent) for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_transfer_receiver = match self
            .transfer
            .get_total_amount_by_receiver(card_number.clone())
            .await
        {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get transfer (received) for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get transfer (received) for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let total_withdraw = match self
            .withdraw
            .get_total_amount_by_card(card_number.clone())
            .await
        {
            Ok(amount) => amount,
            Err(e) => {
                error!("❌ Failed to get withdraw for card {card_number}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to get withdraw for card {}",
                            mask_card_number(&card_number)
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let dashboard = DashboardCardCardNumber {
            total_balance: Some(total_balance),
            total_topup: Some(total_topup),
            total_transaction: Some(total_transaction),
            total_transfer_send: Some(total_transfer_send),
            total_transfer_receiver: Some(total_transfer_receiver),
            total_withdraw: Some(total_withdraw),
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Dashboard for card {} retrieved successfully",
                mask_card_number(&card_number)
            ),
            data: dashboard,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Dashboard for card {} retrieved successfully",
            mask_card_number(&card_number)
        );
        self.tracing_metrics_core
            .complete_tracing_success(
                &tracing_ctx,
                method,
                &format!(
                    "Dashboard for card {} retrieved successfully",
                    mask_card_number(&card_number)
                ),
            )
            .await;

        Ok(response)
    }
}
