mod amount;
mod method;
mod totalamount;

pub use self::amount::{DynMerchantStatsAmountGrpcClient, MerchantStatsAmountGrpcClientTrait};
pub use self::method::{DynMerchantStatsMethodGrpcClient, MerchantStatsMethodGrpcClientTrait};
pub use self::totalamount::{
    DynMerchantStatsTotalAmountGrpcClient, MerchantStatsTotalAmountGrpcClientTrait,
};
