mod auth;
mod card;
mod merchant;
mod role;
mod saldo;
mod topup;
mod transaction;
mod transfer;
mod user;
mod withdraw;

pub use self::auth::AuthGrpcClientService;
pub use self::card::CardGrpcClientService;
pub use self::merchant::MerchantGrpcClientService;
pub use self::role::RoleGrpcClientService;
pub use self::saldo::SaldoGrpcClientService;
pub use self::topup::TopupGrpcClientService;
pub use self::transaction::TransactionGrpcClientService;
pub use self::transfer::TransferGrpcClientService;
pub use self::user::UserGrpcClientService;
pub use self::withdraw::WithdrawGrpcClientService;

use anyhow::{Context, Result};
use shared::config::GrpcClientConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};

use genproto::{
    auth::auth_service_client::AuthServiceClient, card::card_service_client::CardServiceClient,
    merchant::merchant_service_client::MerchantServiceClient,
    role::role_service_client::RoleServiceClient, saldo::saldo_service_client::SaldoServiceClient,
    topup::topup_service_client::TopupServiceClient,
    transaction::transaction_service_client::TransactionServiceClient,
    transfer::transfer_service_client::TransferServiceClient,
    user::user_service_client::UserServiceClient,
    withdraw::withdraw_service_client::WithdrawServiceClient,
};

#[derive(Clone)]
pub struct GrpcClients {
    pub auth: Arc<Mutex<AuthServiceClient<Channel>>>,
    pub card: Arc<Mutex<CardServiceClient<Channel>>>,
    pub merchant: Arc<Mutex<MerchantServiceClient<Channel>>>,
    pub role: Arc<Mutex<RoleServiceClient<Channel>>>,
    pub saldo: Arc<Mutex<SaldoServiceClient<Channel>>>,
    pub topup: Arc<Mutex<TopupServiceClient<Channel>>>,
    pub transaction: Arc<Mutex<TransactionServiceClient<Channel>>>,
    pub transfer: Arc<Mutex<TransferServiceClient<Channel>>>,
    pub user: Arc<Mutex<UserServiceClient<Channel>>>,
    pub withdraw: Arc<Mutex<WithdrawServiceClient<Channel>>>,
}

impl GrpcClients {
    pub async fn init(config: GrpcClientConfig) -> Result<Self> {
        let auth_channel = Self::connect(config.auth, "auth-service").await?;
        let card_channel = Self::connect(config.card, "card-service").await?;
        let merchant_channel = Self::connect(config.merchant, "merchant-service").await?;
        let role_channel = Self::connect(config.role, "role-service").await?;
        let saldo_channel = Self::connect(config.saldo, "saldo-service").await?;
        let topup_channel = Self::connect(config.topup, "topup-service").await?;
        let transaction_channel = Self::connect(config.transaction, "transaction-service").await?;
        let transfer_channel = Self::connect(config.transfer, "transfer-service").await?;
        let user_channel = Self::connect(config.user, "user-service").await?;
        let withdraw_channel = Self::connect(config.withdraw, "withdraw-service").await?;

        Ok(Self {
            auth: Arc::new(Mutex::new(AuthServiceClient::new(auth_channel))),
            card: Arc::new(Mutex::new(CardServiceClient::new(card_channel))),
            merchant: Arc::new(Mutex::new(MerchantServiceClient::new(merchant_channel))),
            role: Arc::new(Mutex::new(RoleServiceClient::new(role_channel))),
            saldo: Arc::new(Mutex::new(SaldoServiceClient::new(saldo_channel))),
            topup: Arc::new(Mutex::new(TopupServiceClient::new(topup_channel))),
            transaction: Arc::new(Mutex::new(TransactionServiceClient::new(
                transaction_channel,
            ))),
            transfer: Arc::new(Mutex::new(TransferServiceClient::new(transfer_channel))),
            user: Arc::new(Mutex::new(UserServiceClient::new(user_channel))),
            withdraw: Arc::new(Mutex::new(WithdrawServiceClient::new(withdraw_channel))),
        })
    }

    async fn connect(addr: String, service: &str) -> Result<Channel> {
        let endpoint = Endpoint::from_shared(addr.clone())
            .with_context(|| format!("Invalid gRPC address for {service}: {addr}"))?;

        endpoint
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {service} at {addr}"))
    }
}
