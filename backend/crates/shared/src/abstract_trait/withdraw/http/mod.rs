mod command;
mod query;
mod stats;
mod statsbycard;

pub use self::command::WithdrawCommandGrpcClientTrait;
pub use self::query::WithdrawQueryGrpcClientTrait;
pub use self::stats::{
    amount::WithdrawStatsAmountGrpcClientTrait, status::WithdrawStatsStatusGrpcClientTrait,
};
pub use self::statsbycard::{
    amount::WithdrawStatsAmountByCardNumberGrpcClientTrait,
    status::WithdrawStatsStatusByCardNumberGrpcClientTrait,
};

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait WithdrawGrpcClientServiceTrait:
    WithdrawCommandGrpcClientTrait
    + WithdrawQueryGrpcClientTrait
    + WithdrawStatsAmountGrpcClientTrait
    + WithdrawStatsStatusGrpcClientTrait
    + WithdrawStatsAmountByCardNumberGrpcClientTrait
    + WithdrawStatsStatusByCardNumberGrpcClientTrait
{
}

pub type DynWithdrawGrpcClientService = Arc<dyn WithdrawGrpcClientServiceTrait + Send + Sync>;
