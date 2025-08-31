use crate::{
    domain::requests::saldo::{
        CreateSaldoRequest, UpdateSaldoBalance, UpdateSaldoRequest, UpdateSaldoWithdraw,
    },
    errors::RepositoryError,
    model::saldo::SaldoModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoCommandRepository = Arc<dyn SaldoCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait SaldoCommandRepositoryTrait {
    async fn create(&self, req: &CreateSaldoRequest) -> Result<SaldoModel, RepositoryError>;
    async fn update(&self, req: &UpdateSaldoRequest) -> Result<SaldoModel, RepositoryError>;
    async fn update_balance(&self, req: &UpdateSaldoBalance)
    -> Result<SaldoModel, RepositoryError>;
    async fn update_withdraw(
        &self,
        req: &UpdateSaldoWithdraw,
    ) -> Result<SaldoModel, RepositoryError>;
    async fn trash(&self, id: i32) -> Result<SaldoModel, RepositoryError>;
    async fn restore(&self, id: i32) -> Result<SaldoModel, RepositoryError>;
    async fn delete_permanent(&self, id: i32) -> Result<(), RepositoryError>;
    async fn restore_all(&self) -> Result<(), RepositoryError>;
    async fn delete_all(&self) -> Result<(), RepositoryError>;
}
