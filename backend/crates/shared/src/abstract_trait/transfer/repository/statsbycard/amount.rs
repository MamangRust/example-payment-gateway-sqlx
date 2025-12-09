use crate::{
    domain::requests::transfer::MonthYearCardNumber,
    errors::RepositoryError,
    model::transfer::{TransferMonthAmount, TransferYearAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountByCardRepository =
    Arc<dyn TransferStatsAmountByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardRepositoryTrait {
    async fn get_monthly_amounts_by_sender_card(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferMonthAmount>, RepositoryError>;
    async fn get_yearly_amounts_by_sender_card(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferYearAmount>, RepositoryError>;
    async fn get_monthly_amounts_by_receiver_card(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferMonthAmount>, RepositoryError>;
    async fn get_yearly_amounts_by_receiver_card(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferYearAmount>, RepositoryError>;
}
