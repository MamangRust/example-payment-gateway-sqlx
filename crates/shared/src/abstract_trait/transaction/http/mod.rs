mod command;
mod query;
mod stats;
mod statsbycard;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::TransactionCommandGrpcClientTrait;
pub use self::query::TransactionQueryGrpcClientTrait;
pub use self::stats::{
    amount::TransactionStatsAmountGrpcClientTrait, method::TransactionStatsMethodGrpcClientTrait,
    status::TransactionStatsStatusGrpcClientTrait,
};
pub use self::statsbycard::{
    amount::TransactionStatsAmountByCardNumberGrpcClientTrait,
    method::TransactionStatsMethodByCardNumberGrpcClientTrait,
    status::TransactionStatsStatusByCardNumberGrpcClientTrait,
};

#[async_trait]
pub trait TransactionGrpcClientServiceTrait:
    TransactionQueryGrpcClientTrait
    + TransactionCommandGrpcClientTrait
    + TransactionStatsAmountGrpcClientTrait
    + TransactionStatsMethodGrpcClientTrait
    + TransactionStatsStatusGrpcClientTrait
    + TransactionStatsAmountByCardNumberGrpcClientTrait
    + TransactionStatsMethodByCardNumberGrpcClientTrait
    + TransactionStatsStatusByCardNumberGrpcClientTrait
{
}

pub type DynTransactionGrpcClientService = Arc<dyn TransactionGrpcClientServiceTrait + Send + Sync>;
