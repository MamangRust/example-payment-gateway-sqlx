use crate::service::{
    AuthGrpcClientService, CardGrpcClientService, GrpcClients, MerchantGrpcClientService,
    RoleGrpcClientService, SaldoGrpcClientService, TopupGrpcClientService,
    TransactionGrpcClientService, TransferGrpcClientService, UserGrpcClientService,
    WithdrawGrpcClientService,
};
use anyhow::Result;
use shared::abstract_trait::{
    auth::http::DynAuthGrpcClient, card::http::DynCardGrpcClientService,
    merchant::http::DynMerchantGrpcClientService, role::http::DynRoleGrpcClientService,
    saldo::http::DynSaldoGrpcClientService, topup::http::DynTopupGrpcClientService,
    transaction::http::DynTransactionGrpcClientService,
    transfer::http::DynTransferGrpcClientService, user::http::DynUserGrpcServiceClient,
    withdraw::http::DynWithdrawGrpcClientService,
};
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
    pub async fn new(clients: GrpcClients) -> Result<Self> {
        let auth_clients: DynAuthGrpcClient =
            Arc::new(AuthGrpcClientService::new(clients.auth).await) as DynAuthGrpcClient;

        let card_clients =
            Arc::new(CardGrpcClientService::new(clients.card).await) as DynCardGrpcClientService;

        let merchant_clients = Arc::new(MerchantGrpcClientService::new(clients.merchant).await)
            as DynMerchantGrpcClientService;
        let role_clients =
            Arc::new(RoleGrpcClientService::new(clients.role).await) as DynRoleGrpcClientService;
        let saldo_clients =
            Arc::new(SaldoGrpcClientService::new(clients.saldo).await) as DynSaldoGrpcClientService;
        let topup_clients =
            Arc::new(TopupGrpcClientService::new(clients.topup).await) as DynTopupGrpcClientService;
        let transaction_clients =
            Arc::new(TransactionGrpcClientService::new(clients.transaction).await)
                as DynTransactionGrpcClientService;
        let transfer_clients = Arc::new(TransferGrpcClientService::new(clients.transfer).await)
            as DynTransferGrpcClientService;
        let user_clients =
            Arc::new(UserGrpcClientService::new(clients.user).await) as DynUserGrpcServiceClient;
        let withdraw_clients = Arc::new(WithdrawGrpcClientService::new(clients.withdraw).await)
            as DynWithdrawGrpcClientService;

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
