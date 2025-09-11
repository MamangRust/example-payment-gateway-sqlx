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
        let auth = std::env::var("AUTH_GRPC_ADDR")
            .context("Missing environment variable: AUTH_GRPC_ADDR")?;

        let card = std::env::var("CARD_GRPC_ADDR")
            .context("Missing environment variable: CARD_GRPC_ADDR")?;

        let merchant = std::env::var("MERCHANT_GRPC_ADDR")
            .context("Missing environment variable: MERCHANT_GRPC_ADDR")?;

        let role = std::env::var("ROLE_GRPC_ADDR")
            .context("Missing environment variable: ROLE_GRPC_ADDR")?;

        let saldo = std::env::var("SALDO_GRPC_ADDR")
            .context("Missing environment variable: SALDO_GRPC_ADDR")?;

        let topup = std::env::var("TOPUP_GRPC_ADDR")
            .context("Missing environment variable: TOPUP_GRPC_ADDR")?;

        let transaction = std::env::var("TRANSACTION_GRPC_ADDR")
            .context("Missing environment variable: TRANSACTION_GRPC_ADDR")?;

        let transfer = std::env::var("TRANSFER_GRPC_ADDR")
            .context("Missing environment variable: TRANSFER_GRPC_ADDR")?;

        let user = std::env::var("USER_GRPC_ADDR")
            .context("Missing environment variable: USER_GRPC_ADDR")?;

        let withdraw = std::env::var("WITHDRAW_GRPC_ADDR")
            .context("Missing environment variable: WITHDRAW_GRPC_ADDR")?;

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
