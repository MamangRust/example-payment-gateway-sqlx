use crate::{
    domain::requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
    errors::RepositoryError,
    model::transaction::TransactionModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionQueryRepository = Arc<dyn TransactionQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionQueryRepositoryTrait {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError>;

    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError>;

    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError>;

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError>;

    async fn find_by_id(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError>;

    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<Vec<TransactionModel>, RepositoryError>;
}
