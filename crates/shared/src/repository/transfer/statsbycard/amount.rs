use crate::{
    domain::requests::transfer::MonthYearCardNumber,
    errors::RepositoryError,
    model::transfer::{TransferMonthAmount, TransferYearAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountByCardNumberRepository =
    Arc<dyn TransferStatsAmountByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardNumberRepositoryTrait {
    async fn get_monthly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferMonthAmount>, RepositoryError>;
    async fn get_yearly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferYearAmount>, RepositoryError>;
    async fn get_monthly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferMonthAmount>, RepositoryError>;
    async fn get_yearly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferYearAmount>, RepositoryError>;
}
