use anyhow::{Context, Result};

#[derive(Clone)]
pub struct GrpcClientConfig {
    pub auth: String,
    pub card: String,
    pub merchant: String,
    pub role: String,
    pub saldo: String,
    pub topup: String,
    pub transaction: String,
    pub transfer: String,
    pub user: String,
    pub withdraw: String,
}

impl GrpcClientConfig {
    pub fn init() -> Result<Self> {
        let auth = std::env::var("GRPC_AUTH_ADDR")
            .context("Missing environment variable: GRPC_AUTH_ADDR")?;

        let card = std::env::var("GRPC_CARD_ADDR")
            .context("Missing environment variable: GRPC_CARD_ADDR")?;

        let merchant = std::env::var("GRPC_MERCHANT_ADDR")
            .context("Missing environment variable: GRPC_MERCHANT_ADDR")?;

        let role = std::env::var("GRPC_ROLE_ADDR")
            .context("Missing environment variable: GRPC_ROLE_ADDR")?;

        let saldo = std::env::var("GRPC_SALDO_ADDR")
            .context("Missing environment variable: GRPC_SALDO_ADDR")?;

        let topup = std::env::var("GRPC_TOPUP_ADDR")
            .context("Missing environment variable: GRPC_TOPUP_ADDR")?;

        let transaction = std::env::var("GRPC_TRANSACTION_ADDR")
            .context("Missing environment variable: GRPC_TRANSACTION_ADDR")?;

        let transfer = std::env::var("GRPC_TRANSFER_ADDR")
            .context("Missing environment variable: GRPC_TRANSFER_ADDR")?;

        let user = std::env::var("GRPC_USER_ADDR")
            .context("Missing environment variable: GRPC_USER_ADDR")?;

        let withdraw = std::env::var("GRPC_WITHDRAW_ADDR")
            .context("Missing environment variable: GRPC_WITHDRAW_ADDR")?;

        Ok(Self {
            auth,
            card,
            merchant,
            role,
            saldo,
            topup,
            transaction,
            transfer,
            user,
            withdraw,
        })
    }
}
