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
use std::time::Duration;

use anyhow::{Context, Result};
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
use shared::config::GrpcClientConfig;
use tonic::transport::{Channel, Endpoint};
use tracing::info;

#[derive(Clone)]
pub struct GrpcClients {
    pub auth: AuthServiceClient<Channel>,
    pub card: CardServiceClient<Channel>,
    pub merchant: MerchantServiceClient<Channel>,
    pub role: RoleServiceClient<Channel>,
    pub saldo: SaldoServiceClient<Channel>,
    pub topup: TopupServiceClient<Channel>,
    pub transaction: TransactionServiceClient<Channel>,
    pub transfer: TransferServiceClient<Channel>,
    pub user: UserServiceClient<Channel>,
    pub withdraw: WithdrawServiceClient<Channel>,
}

impl GrpcClients {
    pub async fn init(config: GrpcClientConfig) -> Result<Self> {
        let auth_channel = Self::connect(&config.auth, "auth-service").await?;
        let card_channel = Self::connect(&config.card, "card-service").await?;
        let merchant_channel = Self::connect(&config.merchant, "merchant-service").await?;
        let role_channel = Self::connect(&config.role, "role-service").await?;
        let saldo_channel = Self::connect(&config.saldo, "saldo-service").await?;
        let topup_channel = Self::connect(&config.topup, "topup-service").await?;
        let transaction_channel = Self::connect(&config.transaction, "transaction-service").await?;
        let transfer_channel = Self::connect(&config.transfer, "transfer-service").await?;
        let user_channel = Self::connect(&config.user, "user-service").await?;
        let withdraw_channel = Self::connect(&config.withdraw, "withdraw-service").await?;

        Ok(Self {
            auth: AuthServiceClient::new(auth_channel),
            card: CardServiceClient::new(card_channel),
            merchant: MerchantServiceClient::new(merchant_channel),
            role: RoleServiceClient::new(role_channel),
            saldo: SaldoServiceClient::new(saldo_channel),
            topup: TopupServiceClient::new(topup_channel),
            transaction: TransactionServiceClient::new(transaction_channel),
            transfer: TransferServiceClient::new(transfer_channel),
            user: UserServiceClient::new(user_channel),
            withdraw: WithdrawServiceClient::new(withdraw_channel),
        })
    }

    async fn connect(addr: &str, service: &str) -> Result<Channel> {
        info!("Connecting (balanced) to {} at {}", service, addr);

        const POOL_SIZE: usize = 10;

        let mut endpoints = Vec::with_capacity(POOL_SIZE);

        for _ in 0..POOL_SIZE {
            let ep = Endpoint::from_shared(addr.to_string())
                .with_context(|| format!("Invalid gRPC address for {service}: {addr}"))?
                .connect_timeout(Duration::from_secs(3))
                .timeout(Duration::from_secs(15))
                .tcp_keepalive(Some(Duration::from_secs(120)))
                .keep_alive_while_idle(true)
                .keep_alive_timeout(Duration::from_secs(10))
                .http2_keep_alive_interval(Duration::from_secs(20))
                .initial_connection_window_size(4 * 1024 * 1024)
                .initial_stream_window_size(2 * 1024 * 1024)
                .concurrency_limit(500)
                .rate_limit(1500, Duration::from_secs(1))
                .tcp_nodelay(true);

            endpoints.push(ep);
        }

        let channel = Channel::balance_list(endpoints.into_iter());

        info!("Successfully created balanced channel for {}", service);
        Ok(channel)
    }
}
