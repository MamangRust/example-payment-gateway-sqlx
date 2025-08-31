use crate::config::grpc::GrpcClientConfig;
use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub grpc_port: u16,
    pub metric_port: u16,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub run_migrations: bool,
    pub port: u16,
    pub auth: ServiceConfig,
    pub user: ServiceConfig,
    pub role: ServiceConfig,
    pub product: ServiceConfig,
    pub order: ServiceConfig,
}
impl Config {
    pub fn init() -> Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").context("Missing environment variable: DATABASE_URL")?;
        let jwt_secret =
            std::env::var("JWT_SECRET").context("Missing environment variable: JWT_SECRET")?;
        let run_migrations_str = std::env::var("RUN_MIGRATIONS")
            .context("Missing environment variable: RUN_MIGRATIONS")?;
        let port_str = std::env::var("PORT").context("Missing environment variable: PORT")?;

        let run_migrations = match run_migrations_str.as_str() {
            "true" => true,
            "false" => false,
            other => {
                return Err(anyhow!(
                    "RUN_MIGRATIONS must be 'true' or 'false', got '{}'",
                    other
                ));
            }
        };

        let port = port_str
            .parse::<u16>()
            .context("PORT must be a valid u16 integer")?;

        // user
        let auth_grpc_port = std::env::var("AUTH_GRPC_PORT")
            .context("Missing environment variable: AUTH_GRPC_PORT")?
            .parse::<u16>()
            .context("AUTH_GRPC_PORT must be a valid u16 integer")?;

        let auth_metric_port = std::env::var("AUTH_METRIC_PORT")
            .context("Missing environment variable: AUTH_METRIC_PORT")?
            .parse::<u16>()
            .context("AUTH_METRIC_PORT must be a valid u16 integer")?;

        // user
        let user_grpc_port = std::env::var("USER_GRPC_PORT")
            .context("Missing environment variable: USER_GRPC_PORT")?
            .parse::<u16>()
            .context("USER_GRPC_PORT must be a valid u16 integer")?;

        let user_metric_port = std::env::var("USER_METRIC_PORT")
            .context("Missing environment variable: USER_METRIC_PORT")?
            .parse::<u16>()
            .context("USER_METRIC_PORT must be a valid u16 integer")?;

        // role
        let role_grpc_port = std::env::var("ROLE_GRPC_PORT")
            .context("Missing environment variable: ROLE_GRPC_PORT")?
            .parse::<u16>()
            .context("ROLE_GRPC_PORT must be a valid u16 integer")?;

        let role_metric_port = std::env::var("ROLE_METRIC_PORT")
            .context("Missing environment variable: ROLE_METRIC_PORT")?
            .parse::<u16>()
            .context("ROLE_METRIC_PORT must be a valid u16 integer")?;

        // product
        let product_grpc_port = std::env::var("PRODUCT_GRPC_PORT")
            .context("Missing environment variable: PRODUCT_GRPC_PORT")?
            .parse::<u16>()
            .context("PRODUCT_GRPC_PORT must be a valid u16 integer")?;

        let product_metric_port = std::env::var("PRODUCT_METRIC_PORT")
            .context("Missing environment variable: PRODUCT_METRIC_PORT")?
            .parse::<u16>()
            .context("PRODUCT_METRIC_PORT must be a valid u16 integer")?;

        // order
        let order_grpc_port = std::env::var("ORDER_GRPC_PORT")
            .context("Missing environment variable: ORDER_GRPC_PORT")?
            .parse::<u16>()
            .context("ORDER_GRPC_PORT must be a valid u16 integer")?;

        let order_metric_port = std::env::var("ORDER_METRIC_PORT")
            .context("Missing environment variable: ORDER_METRIC_PORT")?
            .parse::<u16>()
            .context("ORDER_METRIC_PORT must be a valid u16 integer")?;

        Ok(Self {
            database_url,
            jwt_secret,
            run_migrations,
            port,
            auth: ServiceConfig {
                grpc_port: auth_grpc_port,
                metric_port: auth_metric_port,
            },
            user: ServiceConfig {
                grpc_port: user_grpc_port,
                metric_port: user_metric_port,
            },
            role: ServiceConfig {
                grpc_port: role_grpc_port,
                metric_port: role_metric_port,
            },
            product: ServiceConfig {
                grpc_port: product_grpc_port,
                metric_port: product_metric_port,
            },
            order: ServiceConfig {
                grpc_port: order_grpc_port,
                metric_port: order_metric_port,
            },
        })
    }
}
