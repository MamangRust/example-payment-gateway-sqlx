mod command;
mod dashboard;
mod query;
mod stats;
mod statsbycard;

pub use self::command::CardCommandGrpcClientTrait;
pub use self::dashboard::CardDashboardGrpcClientTrait;
pub use self::query::CardQueryGrpcClientTrait;
pub use self::stats::{
    balance::CardStatsBalanceGrpcClientTrait, topup::CardStatsTopupGrpcClientTrait,
    transaction::CardStatsTransactionGrpcClientTrait, transfer::CardStatsTransferGrpcClientTrait,
    withdraw::CardStatsWithdrawGrpcClientTrait,
};
pub use self::statsbycard::{
    balance::CardStatsBalanceByCardGrpcClientTrait, topup::CardStatsTopupByCardGrpcClientTrait,
    transaction::CardStatsTransactionByCardGrpcClientTrait,
    transfer::CardStatsTransferByCardGrpcClientTrait,
    withdraw::CardStatsWithdrawByCardGrpcClientTrait,
};

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CardGrpcClientServiceTrait:
    CardCommandGrpcClientTrait
    + CardQueryGrpcClientTrait
    + CardStatsBalanceGrpcClientTrait
    + CardStatsTopupGrpcClientTrait
    + CardStatsTransactionGrpcClientTrait
    + CardStatsTransferGrpcClientTrait
    + CardStatsWithdrawGrpcClientTrait
    + CardStatsBalanceByCardGrpcClientTrait
    + CardStatsTopupByCardGrpcClientTrait
    + CardStatsTransactionByCardGrpcClientTrait
    + CardStatsTransferByCardGrpcClientTrait
    + CardStatsWithdrawByCardGrpcClientTrait
    + CardDashboardGrpcClientTrait
{
}

pub type DynCardGrpcClientService = Arc<dyn CardGrpcClientServiceTrait + Send + Sync>;
