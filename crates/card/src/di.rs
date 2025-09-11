use std::sync::Arc;

use anyhow::Result;
use shared::{
    abstract_trait::{
        card::{
            repository::{
                command::DynCardCommandRepository,
                dashboard::{
                    balance::DynCardDashboardBalanceRepository,
                    topup::DynCardDashboardTopupRepository,
                    transaction::DynCardDashboardTransactionRepository,
                    transfer::DynCardDashboardTransferRepository,
                    withdraw::DynCardDashboardWithdrawRepository,
                },
                query::DynCardQueryRepository,
                stats::{
                    balance::DynCardStatsBalanceRepository, topup::DynCardStatsTopupRepository,
                    transaction::DynCardStatsTransactionRepository,
                    transfer::DynCardStatsTransferRepository,
                    withdraw::DynCardStatsWithdrawRepository,
                },
                statsbycard::{
                    balance::DynCardStatsBalanceByCardRepository,
                    topup::DynCardStatsTopupByCardRepository,
                    transaction::DynCardStatsTransactionByCardRepository,
                    transfer::DynCardStatsTransferByCardRepository,
                    withdraw::DynCardStatsWithdrawByCardRepository,
                },
            },
            service::{
                command::DynCardCommandService,
                dashboard::DynCardDashboardService,
                query::DynCardQueryService,
                stats::{
                    balance::DynCardStatsBalanceService, topup::DynCardStatsTopupService,
                    transaction::DynCardStatsTransactionService,
                    transfer::DynCardStatsTransferService, withdraw::DynCardStatsWithdrawService,
                },
                statsbycard::{
                    balance::DynCardStatsBalanceByCardService,
                    topup::DynCardStatsTopupByCardService,
                    transaction::DynCardStatsTransactionByCardService,
                    transfer::DynCardStatsTransferByCardService,
                    withdraw::DynCardStatsWithdrawByCardService,
                },
            },
        },
        user::repository::query::DynUserQueryRepository,
    },
    config::ConnectionPool,
    repository::{
        card::{
            command::CardCommandRepository,
            dashboard::{
                balance::CardDashboardBalanceRepository, topup::CardDashboardTopupRepository,
                transaction::CardDashboardTransactionRepository,
                transfer::CardDashboardTransferRepository,
                withdraw::CardDashboardWithdrawRepository,
            },
            query::CardQueryRepository,
            stats::{
                balance::CardStatsBalanceRepository, topup::CardStatsTopupRepository,
                transaction::CardStatsTransactionRepository, transfer::CardStatsTransferRepository,
                withdraw::CardStatsWithdrawRepository,
            },
            statsbycard::{
                balance::CardStatsBalanceByCardRepository, topup::CardStatsTopupByCardRepository,
                transaction::CardStatsTransactionByCardRepository,
                transfer::CardStatsTransferByCardRepository,
                withdraw::CardStatsWithdrawByCardRepository,
            },
        },
        user::query::UserQueryRepository,
    },
    service::card::{
        command::CardCommandService,
        dashboard::CardDashboardService,
        query::CardQueryService,
        stats::{
            balance::CardStatsBalanceService, topup::CardStatsTopupService,
            transaction::CardStatsTransactionService, transfer::CardStatsTransferService,
            withdraw::CardStatsWithdrawService,
        },
        statsbycard::{
            balance::CardStatsBalanceByCardService, topup::CardStatsTopupByCardService,
            transaction::CardStatsTransactionByCardService,
            transfer::CardStatsTransferByCardService, withdraw::CardStatsWithdrawByCardService,
        },
    },
};

#[derive(Clone)]
pub struct CardQueryDeps {
    pub query: DynCardQueryRepository,
    pub service: DynCardQueryService,
}

impl CardQueryDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let query = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let service = Arc::new(CardQueryService::new(query.clone()).await) as DynCardQueryService;
        Self { query, service }
    }
}

#[derive(Clone)]
pub struct CardCommandDeps {
    pub command: DynCardCommandRepository,
    pub service: DynCardCommandService,
}

impl CardCommandDeps {
    pub async fn new(db: ConnectionPool, user_query: DynUserQueryRepository) -> Self {
        let command = Arc::new(CardCommandRepository::new(db.clone())) as DynCardCommandRepository;
        let service = Arc::new(CardCommandService::new(user_query, command.clone()).await)
            as DynCardCommandService;
        Self { command, service }
    }
}

#[derive(Clone)]
pub struct CardDashboardDeps {
    pub service: DynCardDashboardService,
}

impl CardDashboardDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let balance = Arc::new(CardDashboardBalanceRepository::new(db.clone()))
            as DynCardDashboardBalanceRepository;
        let topup = Arc::new(CardDashboardTopupRepository::new(db.clone()))
            as DynCardDashboardTopupRepository;
        let trx = Arc::new(CardDashboardTransactionRepository::new(db.clone()))
            as DynCardDashboardTransactionRepository;
        let transfer = Arc::new(CardDashboardTransferRepository::new(db.clone()))
            as DynCardDashboardTransferRepository;
        let withdraw = Arc::new(CardDashboardWithdrawRepository::new(db.clone()))
            as DynCardDashboardWithdrawRepository;

        let service =
            Arc::new(CardDashboardService::new(balance, topup, trx, transfer, withdraw).await)
                as DynCardDashboardService;

        Self { service }
    }
}

#[derive(Clone)]
pub struct CardStatsService {
    pub balance: DynCardStatsBalanceService,
    pub topup: DynCardStatsTopupService,
    pub transaction: DynCardStatsTransactionService,
    pub transfer: DynCardStatsTransferService,
    pub withdraw: DynCardStatsWithdrawService,
}

#[derive(Clone)]
pub struct CardStatsByCardService {
    pub balance: DynCardStatsBalanceByCardService,
    pub topup: DynCardStatsTopupByCardService,
    pub transaction: DynCardStatsTransactionByCardService,
    pub transfer: DynCardStatsTransferByCardService,
    pub withdraw: DynCardStatsWithdrawByCardService,
}

#[derive(Clone)]
pub struct CardStatsDeps {
    pub stats: CardStatsService,
    pub stats_bycard: CardStatsByCardService,
}

impl CardStatsDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let balance_repo =
            Arc::new(CardStatsBalanceRepository::new(db.clone())) as DynCardStatsBalanceRepository;
        let topup_repo =
            Arc::new(CardStatsTopupRepository::new(db.clone())) as DynCardStatsTopupRepository;
        let trx_repo = Arc::new(CardStatsTransactionRepository::new(db.clone()))
            as DynCardStatsTransactionRepository;
        let transfer_repo = Arc::new(CardStatsTransferRepository::new(db.clone()))
            as DynCardStatsTransferRepository;
        let withdraw_repo = Arc::new(CardStatsWithdrawRepository::new(db.clone()))
            as DynCardStatsWithdrawRepository;

        let stats = CardStatsService {
            balance: Arc::new(CardStatsBalanceService::new(balance_repo).await)
                as DynCardStatsBalanceService,
            topup: Arc::new(CardStatsTopupService::new(topup_repo).await)
                as DynCardStatsTopupService,
            transaction: Arc::new(CardStatsTransactionService::new(trx_repo).await)
                as DynCardStatsTransactionService,
            transfer: Arc::new(CardStatsTransferService::new(transfer_repo).await)
                as DynCardStatsTransferService,
            withdraw: Arc::new(CardStatsWithdrawService::new(withdraw_repo).await)
                as DynCardStatsWithdrawService,
        };

        let balance_bycard_repo = Arc::new(CardStatsBalanceByCardRepository::new(db.clone()))
            as DynCardStatsBalanceByCardRepository;
        let topup_bycard_repo = Arc::new(CardStatsTopupByCardRepository::new(db.clone()))
            as DynCardStatsTopupByCardRepository;
        let trx_bycard_repo = Arc::new(CardStatsTransactionByCardRepository::new(db.clone()))
            as DynCardStatsTransactionByCardRepository;
        let transfer_bycard_repo = Arc::new(CardStatsTransferByCardRepository::new(db.clone()))
            as DynCardStatsTransferByCardRepository;
        let withdraw_bycard_repo = Arc::new(CardStatsWithdrawByCardRepository::new(db.clone()))
            as DynCardStatsWithdrawByCardRepository;

        let stats_bycard = CardStatsByCardService {
            balance: Arc::new(CardStatsBalanceByCardService::new(balance_bycard_repo).await)
                as DynCardStatsBalanceByCardService,
            topup: Arc::new(CardStatsTopupByCardService::new(topup_bycard_repo).await)
                as DynCardStatsTopupByCardService,
            transaction: Arc::new(CardStatsTransactionByCardService::new(trx_bycard_repo).await)
                as DynCardStatsTransactionByCardService,
            transfer: Arc::new(CardStatsTransferByCardService::new(transfer_bycard_repo).await)
                as DynCardStatsTransferByCardService,
            withdraw: Arc::new(CardStatsWithdrawByCardService::new(withdraw_bycard_repo).await)
                as DynCardStatsWithdrawByCardService,
        };

        Self {
            stats,
            stats_bycard,
        }
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub card_query: CardQueryDeps,
    pub card_command: CardCommandDeps,
    pub card_dashboard: CardDashboardDeps,
    pub card_stats: CardStatsDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let user_query_repo =
            Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;

        let card_query = CardQueryDeps::new(db.clone()).await;
        let card_command = CardCommandDeps::new(db.clone(), user_query_repo).await;
        let card_dashboard = CardDashboardDeps::new(db.clone()).await;
        let card_stats = CardStatsDeps::new(db.clone()).await;

        Ok(Self {
            card_query,
            card_command,
            card_dashboard,
            card_stats,
        })
    }
}
