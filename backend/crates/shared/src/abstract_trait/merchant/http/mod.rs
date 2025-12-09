mod command;
mod query;
mod stats;
mod statsbyapikey;
mod statsbymerchant;
mod transactions;

pub use self::command::MerchantCommandGrpcClientTrait;
pub use self::query::MerchantQueryGrpcClientTrait;
pub use self::stats::{
    amount::MerchantStatsAmountGrpcClientTrait, method::MerchantStatsMethodGrpcClientTrait,
    totalamount::MerchantStatsTotalAmountGrpcClientTrait,
};
pub use self::statsbyapikey::{
    amount::MerchantStatsAmountByApiKeyGrpcClientTrait,
    method::MerchantStatsMethodByApiKeyGrpcClientTrait,
    totalamount::MerchantStatsTotalAmountByApiKeyGrpcClientTrait,
};
pub use self::statsbymerchant::{
    amount::MerchantStatsAmountByMerchantGrpcClientTrait,
    method::MerchantStatsMethodByMerchantGrpcClientTrait,
    totalamount::MerchantStatsTotalAmountByMerchantGrpcClientTrait,
};
pub use self::transactions::MerchantTransactionGrpcClientTrait;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait MerchantGrpcClientServiceTrait:
    MerchantQueryGrpcClientTrait
    + MerchantTransactionGrpcClientTrait
    + MerchantCommandGrpcClientTrait
    + MerchantStatsAmountGrpcClientTrait
    + MerchantStatsMethodGrpcClientTrait
    + MerchantStatsTotalAmountGrpcClientTrait
    + MerchantStatsAmountByMerchantGrpcClientTrait
    + MerchantStatsMethodByMerchantGrpcClientTrait
    + MerchantStatsTotalAmountByMerchantGrpcClientTrait
    + MerchantStatsAmountByApiKeyGrpcClientTrait
    + MerchantStatsMethodByApiKeyGrpcClientTrait
    + MerchantStatsTotalAmountByApiKeyGrpcClientTrait
{
}

pub type DynMerchantGrpcClientService = Arc<dyn MerchantGrpcClientServiceTrait + Send + Sync>;
