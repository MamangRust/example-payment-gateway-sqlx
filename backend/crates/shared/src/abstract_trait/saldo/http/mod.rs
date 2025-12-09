mod command;
mod query;
mod stats;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::SaldoCommandGrpcClientTrait;
pub use self::query::SaldoQueryGrpcClientTrait;
pub use self::stats::{
    balance::SaldoBalanceGrpcClientTrait, total::SaldoTotalBalanceGrpcClientTrait,
};

#[async_trait]
pub trait SaldoGrpcClientServiceTrait:
    SaldoQueryGrpcClientTrait
    + SaldoCommandGrpcClientTrait
    + SaldoBalanceGrpcClientTrait
    + SaldoTotalBalanceGrpcClientTrait
{
}

pub type DynSaldoGrpcClientService = Arc<dyn SaldoGrpcClientServiceTrait + Send + Sync>;
