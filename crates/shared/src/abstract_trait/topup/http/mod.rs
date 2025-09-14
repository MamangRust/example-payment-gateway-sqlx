mod command;
mod query;
mod stats;
mod statsbycard;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::TopupCommandGrpcClientTrait;
pub use self::query::TopupQueryGrpcClientTrait;
pub use self::stats::{
    amount::TopupStatsAmountGrpcClientTrait, method::TopupStatsMethodGrpcClientTrait,
    status::TopupStatsStatusGrpcClientTrait,
};
pub use self::statsbycard::{
    amount::TopupStatsAmountByCardNumberGrpcClientTrait,
    method::TopupStatsMethodByCardNumberGrpcClientTrait,
    status::TopupStatsStatusByCardNumberGrpcClientTrait,
};

#[async_trait]
pub trait TopupGrpcClientServiceTrait:
    TopupQueryGrpcClientTrait
    + TopupCommandGrpcClientTrait
    + TopupStatsAmountGrpcClientTrait
    + TopupStatsMethodGrpcClientTrait
    + TopupStatsStatusGrpcClientTrait
    + TopupStatsAmountByCardNumberGrpcClientTrait
    + TopupStatsMethodByCardNumberGrpcClientTrait
    + TopupStatsStatusByCardNumberGrpcClientTrait
{
}

pub type DynTopupGrpcClientService = Arc<dyn TopupGrpcClientServiceTrait + Send + Sync>;
