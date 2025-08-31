use crate::{
    domain::requests::saldo::FindAllSaldos, errors::RepositoryError, model::saldo::SaldoModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoQueryRepository = Arc<dyn SaldoQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait SaldoQueryRepositoryTrait {
    async fn find_all(
        &self,
        request: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError>;
    async fn find_active(
        &self,
        request: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError>;
    async fn find_trashed(
        &self,
        request: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<SaldoModel, RepositoryError>;
}
