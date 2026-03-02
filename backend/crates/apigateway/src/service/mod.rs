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
use shared::config::{GrpcClientConfig, GrpcServiceEndpoints};
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
    pub async fn init(config: GrpcServiceEndpoints) -> Result<Self> {
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

        let config_grpc = GrpcClientConfig::from_env()?;

        let mut endpoints = Vec::with_capacity(config_grpc.pool_size);

        for _ in 0..config_grpc.pool_size {
            let ep = Endpoint::from_shared(addr.to_string())
                .with_context(|| format!("Invalid gRPC address for {service}: {addr}"))?
                .connect_timeout(config_grpc.connect_timeout())
                .timeout(config_grpc.request_timeout())
                .tcp_keepalive(config_grpc.tcp_keepalive())
                .keep_alive_while_idle(config_grpc.keep_alive_while_idle)
                .keep_alive_timeout(config_grpc.keepalive_timeout())
                .http2_keep_alive_interval(config_grpc.http2_keepalive_interval())
                .initial_connection_window_size(
                    config_grpc.initial_connection_window_size_mb * 1024 * 1024,
                )
                .initial_stream_window_size(config_grpc.initial_stream_window_size_mb * 1024 * 1024)
                .concurrency_limit(config_grpc.concurrency_per_connection)
                .rate_limit(
                    config_grpc.rate_limit_per_sec,
                    config_grpc.rate_limit_duration(),
                )
                .tcp_nodelay(config_grpc.tcp_nodelay);

            endpoints.push(ep);
        }

        let channel = Channel::balance_list(endpoints.into_iter());

        info!(
            "Successfully created balanced channel for {} (pool={}, concurrency={}, rate_limit={}/s)",
            service,
            config_grpc.pool_size,
            config_grpc.concurrency_per_connection,
            config_grpc.rate_limit_per_sec
        );

        Ok(channel)
    }
}
