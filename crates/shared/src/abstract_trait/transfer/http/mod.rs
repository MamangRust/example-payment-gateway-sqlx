mod command;
mod query;
mod stats;
mod statsbycard;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::TransferCommandGrpcClientTrait;
pub use self::query::TransferQueryGrpcClientTrait;
pub use self::stats::{
    amount::TransferStatsAmountGrpcClientTrait, status::TransferStatsStatusGrpcClientTrait,
};
pub use self::statsbycard::{
    amount::TransferStatsAmountByCardNumberGrpcClientTrait,
    status::TransferStatsStatusByCardNumberGrpcClientTrait,
};

#[async_trait]
pub trait TransferGrpcClientServiceTrait:
    TransferCommandGrpcClientTrait
    + TransferQueryGrpcClientTrait
    + TransferStatsAmountGrpcClientTrait
    + TransferStatsStatusGrpcClientTrait
    + TransferStatsAmountByCardNumberGrpcClientTrait
    + TransferStatsStatusByCardNumberGrpcClientTrait
{
}

pub type DynTransferGrpcClientService = Arc<dyn TransferGrpcClientServiceTrait + Send + Sync>;
