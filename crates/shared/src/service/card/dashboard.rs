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
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardDashboardService {
    balance: DynCardDashboardBalanceRepository,
    topup: DynCardDashboardTopupRepository,
    transaction: DynCardDashboardTransactionRepository,
    transfer: DynCardDashboardTransferRepository,
    withdraw: DynCardDashboardWithdrawRepository,
}

impl CardDashboardService {
    pub async fn new(
        balance: DynCardDashboardBalanceRepository,
        topup: DynCardDashboardTopupRepository,
        transaction: DynCardDashboardTransactionRepository,
        transfer: DynCardDashboardTransferRepository,
        withdraw: DynCardDashboardWithdrawRepository,
    ) -> Self {
        Self {
            balance,
            topup,
            transaction,
            transfer,
            withdraw,
        }
    }
}

#[async_trait]
impl CardDashboardServiceTrait for CardDashboardService {
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, ServiceError> {
        info!("📊 Fetching global dashboard statistics (strict mode)");

        let total_balance = self.balance.get_total_balance().await.map_err(|e| {
            error!("❌ Failed to get total balance: {e:?}");
            ServiceError::Repo(e)
        })?;

        let total_topup = self.topup.get_total_amount().await.map_err(|e| {
            error!("❌ Failed to get total top-up: {e:?}");
            ServiceError::Repo(e)
        })?;

        let total_transaction = self.transaction.get_total_amount().await.map_err(|e| {
            error!("❌ Failed to get total transaction: {e:?}");
            ServiceError::Repo(e)
        })?;

        let total_transfer = self.transfer.get_total_amount().await.map_err(|e| {
            error!("❌ Failed to get total transfer: {e:?}");
            ServiceError::Repo(e)
        })?;

        let total_withdraw = self.withdraw.get_total_amount().await.map_err(|e| {
            error!("❌ Failed to get total withdraw: {e:?}");
            ServiceError::Repo(e)
        })?;

        let dashboard = DashboardCard {
            total_balance: Some(total_balance),
            total_topup: Some(total_topup),
            total_transaction: Some(total_transaction),
            total_transfer: Some(total_transfer),
            total_withdraw: Some(total_withdraw),
        };

        info!("✅ Global dashboard retrieved successfully");
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Global dashboard retrieved successfully".to_string(),
            data: dashboard,
        })
    }

    async fn get_dashboard_bycard(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, ServiceError> {
        info!("💳📊 Fetching dashboard for card: {}", card_number);

        let total_balance = self
            .balance
            .get_total_balance_by_card(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get balance for card {card_number}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let total_topup = self
            .topup
            .get_total_amount_by_card(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get top-up for card {card_number}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let total_transaction = self
            .transaction
            .get_total_amount_by_card(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get transaction for card {card_number}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let total_transfer_send = self
            .transfer
            .get_total_amount_by_sender(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get transfer (sent) for card {card_number}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let total_transfer_receiver = self
            .transfer
            .get_total_amount_by_receiver(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get transfer (received) for card {card_number}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let total_withdraw = self
            .withdraw
            .get_total_amount_by_card(card_number.clone())
            .await
            .map_err(|e| {
                error!("❌ Failed to get withdraw for card {card_number}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let dashboard = DashboardCardCardNumber {
            total_balance: Some(total_balance),
            total_topup: Some(total_topup),
            total_transaction: Some(total_transaction),
            total_transfer_send: Some(total_transfer_send),
            total_transfer_receiver: Some(total_transfer_receiver),
            total_withdraw: Some(total_withdraw),
        };

        info!("✅ Dashboard for card {card_number} retrieved successfully",);
        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Dashboard for card {card_number} retrieved successfully"),
            data: dashboard,
        })
    }
}
