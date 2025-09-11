use anyhow::Result;
use shared::abstract_trait::auth::http::DynAuthGrpcClient;
use std::sync::Arc;

use crate::service::{
    AuthGrpcClientService, CardGrpcClientService, GrpcClients, MerchantGrpcClientService,
    RoleGrpcClientService, SaldoGrpcClientService, TopupGrpcClientService,
    TransactionGrpcClientService, TransferGrpcClientService, UserGrpcClientService,
    WithdrawGrpcClientService,
};

#[derive(Clone)]
pub struct DependenciesInject {
    pub auth_clients: DynAuthGrpcClient,
    pub card_clients: Arc<CardGrpcClientService>,
    pub merchant_clients: Arc<MerchantGrpcClientService>,
    pub role_clients: Arc<RoleGrpcClientService>,
    pub saldo_clients: Arc<SaldoGrpcClientService>,
    pub topup_clients: Arc<TopupGrpcClientService>,
    pub transaction_clients: Arc<TransactionGrpcClientService>,
    pub transfer_clients: Arc<TransferGrpcClientService>,
    pub user_clients: Arc<UserGrpcClientService>,
    pub withdraw_clients: Arc<WithdrawGrpcClientService>,
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

        let card_clients = Arc::new(CardGrpcClientService::new(clients.card).await);

        let merchant_clients = Arc::new(MerchantGrpcClientService::new(clients.merchant).await);
        let role_clients = Arc::new(RoleGrpcClientService::new(clients.role).await);
        let saldo_clients = Arc::new(SaldoGrpcClientService::new(clients.saldo).await);
        let topup_clients = Arc::new(TopupGrpcClientService::new(clients.topup).await);
        let transaction_clients =
            Arc::new(TransactionGrpcClientService::new(clients.transaction).await);
        let transfer_clients = Arc::new(TransferGrpcClientService::new(clients.transfer).await);
        let user_clients = Arc::new(UserGrpcClientService::new(clients.user).await);
        let withdraw_clients = Arc::new(WithdrawGrpcClientService::new(clients.withdraw).await);

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
