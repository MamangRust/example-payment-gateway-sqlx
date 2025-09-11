use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub run_migrations: bool,
    pub port: u16,
    pub auth: ServiceConfig,
    pub card: ServiceConfig,
    pub merchant: ServiceConfig,
    pub role: ServiceConfig,
    pub saldo: ServiceConfig,
    pub topup: ServiceConfig,
    pub transaction: ServiceConfig,
    pub transfer: ServiceConfig,
    pub user: ServiceConfig,
    pub withdraw: ServiceConfig,
}

impl Config {
    pub fn init() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").context("Missing env: DATABASE_URL")?;
        let jwt_secret = std::env::var("JWT_SECRET").context("Missing env: JWT_SECRET")?;
        let run_migrations_str =
            std::env::var("RUN_MIGRATIONS").context("Missing env: RUN_MIGRATIONS")?;
        let port_str = std::env::var("PORT").context("Missing env: PORT")?;

        let run_migrations = match run_migrations_str.as_str() {
            "true" => true,
            "false" => false,
            other => {
                return Err(anyhow!(
                    "RUN_MIGRATIONS must be 'true' or 'false', got '{other}'",
                ));
            }
        };

        let port = port_str
            .parse::<u16>()
            .context("PORT must be a valid u16 integer")?;

        Ok(Self {
            database_url,
            jwt_secret,
            run_migrations,
            port,
            auth: ServiceConfig::from_env("AUTH")?,
            card: ServiceConfig::from_env("CARD")?,
            merchant: ServiceConfig::from_env("MERCHANT")?,
            role: ServiceConfig::from_env("ROLE")?,
            saldo: ServiceConfig::from_env("SALDO")?,
            topup: ServiceConfig::from_env("TOPUP")?,
            transaction: ServiceConfig::from_env("TRANSACTION")?,
            transfer: ServiceConfig::from_env("TRANSFER")?,
            user: ServiceConfig::from_env("USER")?,
            withdraw: ServiceConfig::from_env("WITHDRAW")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub grpc_port: u16,
    pub metric_port: u16,
}

impl ServiceConfig {
    pub fn from_env(prefix: &str) -> Result<Self> {
        let grpc_port = std::env::var(format!("{}_GRPC_PORT", prefix))
            .context(format!("Missing env: {prefix}_GRPC_PORT"))?
            .parse::<u16>()
            .context(format!("{prefix}_GRPC_PORT must be a valid u16 integer",))?;

        let metric_port = std::env::var(format!("{}_METRIC_PORT", prefix))
            .context(format!("Missing env: {prefix}_METRIC_PORT"))?
            .parse::<u16>()
            .context(format!("{prefix}_METRIC_PORT must be a valid u16 integer",))?;

        Ok(Self {
            grpc_port,
            metric_port,
        })
    }
}
