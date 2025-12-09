mod command;
mod query;

pub use self::command::UserCommandGrpcClientTrait;
pub use self::query::UserQueryGrpcClientTrait;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserGrpcClientServiceTrait:
    UserCommandGrpcClientTrait + UserQueryGrpcClientTrait
{
}

pub type DynUserGrpcServiceClient = Arc<dyn UserGrpcClientServiceTrait + Send + Sync>;
