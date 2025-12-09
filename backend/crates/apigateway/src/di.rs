use crate::service::{
    AuthGrpcClientService, CardGrpcClientService, GrpcClients, MerchantGrpcClientService,
    RoleGrpcClientService, SaldoGrpcClientService, TopupGrpcClientService,
    TransactionGrpcClientService, TransferGrpcClientService, UserGrpcClientService,
    WithdrawGrpcClientService,
};
use anyhow::{Context, Result};
use shared::abstract_trait::{
    auth::http::DynAuthGrpcClient, card::http::DynCardGrpcClientService,
    merchant::http::DynMerchantGrpcClientService, role::http::DynRoleGrpcClientService,
    saldo::http::DynSaldoGrpcClientService, topup::http::DynTopupGrpcClientService,
    transaction::http::DynTransactionGrpcClientService,
    transfer::http::DynTransferGrpcClientService, user::http::DynUserGrpcServiceClient,
    withdraw::http::DynWithdrawGrpcClientService,
};
use shared::cache::CacheStore;
use shared::config::RedisPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct DependenciesInject {
    pub auth_clients: DynAuthGrpcClient,
    pub card_clients: DynCardGrpcClientService,
    pub merchant_clients: DynMerchantGrpcClientService,
    pub role_clients: DynRoleGrpcClientService,
    pub saldo_clients: DynSaldoGrpcClientService,
    pub topup_clients: DynTopupGrpcClientService,
    pub transaction_clients: DynTransactionGrpcClientService,
    pub transfer_clients: DynTransferGrpcClientService,
    pub user_clients: DynUserGrpcServiceClient,
    pub withdraw_clients: DynWithdrawGrpcClientService,
}

impl std::fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("auth_service", &"DynAuthService")
            .field("card_service", &"CardService")
            .field("merchant_service", &"MerchantService")
            .field("role_service", &"RoleService")
            .field("saldo_service", &"SaldoService")
            .field("topup_service", &"TopupService")
            .field("transaction_service", &"TransactionService")
            .field("transfer_service", &"TransferService")
            .field("user_service", &"UserService")
            .field("withdraw_service", &"WithdrawService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(clients: GrpcClients, redis_client: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis_client.pool.clone()));

        let auth_clients: DynAuthGrpcClient = Arc::new(
            AuthGrpcClientService::new(clients.auth.clone())
                .context("failed initialize auth grpc service")?,
        ) as DynAuthGrpcClient;

        let card_clients = Arc::new(
            CardGrpcClientService::new(clients.card.clone(), cache_store.clone())
                .context("failed initialize card grpc service")?,
        ) as DynCardGrpcClientService;

        let merchant_clients = Arc::new(
            MerchantGrpcClientService::new(clients.merchant.clone(), cache_store.clone())
                .context("failed initialize merchant grpc service")?,
        ) as DynMerchantGrpcClientService;

        let role_clients = Arc::new(
            RoleGrpcClientService::new(clients.role.clone(), cache_store.clone())
                .context("failed initialize role grpc service")?,
        ) as DynRoleGrpcClientService;

        let saldo_clients = Arc::new(
            SaldoGrpcClientService::new(clients.saldo.clone(), cache_store.clone())
                .context("failed initialize saldo grpc service")?,
        ) as DynSaldoGrpcClientService;

        let topup_clients = Arc::new(
            TopupGrpcClientService::new(clients.topup.clone(), cache_store.clone())
                .context("failed initialize topup grpc service")?,
        ) as DynTopupGrpcClientService;

        let transaction_clients = Arc::new(
            TransactionGrpcClientService::new(clients.transaction.clone(), cache_store.clone())
                .context("failed initialize transaction grpc service")?,
        ) as DynTransactionGrpcClientService;

        let transfer_clients = Arc::new(
            TransferGrpcClientService::new(clients.transfer.clone(), cache_store.clone())
                .context("failed initialize transfer grpc service")?,
        ) as DynTransferGrpcClientService;

        let user_clients = Arc::new(
            UserGrpcClientService::new(clients.user.clone(), cache_store.clone())
                .context("failed initialize user grpc service")?,
        ) as DynUserGrpcServiceClient;

        let withdraw_clients = Arc::new(
            WithdrawGrpcClientService::new(clients.withdraw.clone(), cache_store.clone())
                .context("failed initialize withdraw grpc service")?,
        ) as DynWithdrawGrpcClientService;

        Ok(Self {
            auth_clients,
            card_clients,
            merchant_clients,
            role_clients,
            saldo_clients,
            topup_clients,
            transaction_clients,
            transfer_clients,
            user_clients,
            withdraw_clients,
        })
    }
}
