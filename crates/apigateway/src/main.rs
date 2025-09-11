use anyhow::{Context, Result};
use apigateway::{handler::AppRouter, state::AppState};
use dotenv::dotenv;
use shared::{config::Config, utils::Logger};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let is_dev = std::env::var("DEV_MODE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let _logger = Logger::new("apigateway", is_dev);

    let config = Config::init().context("Failed to load configuration")?;

    let port = config.port;

    let state = AppState::new(&config.jwt_secret)
        .await
        .context("Failed to create AppState")?;

    println!("🚀 Server started successfully");

    AppRouter::serve(port, state)
        .await
        .context("Failed to start server")?;

    info!("Shutting down servers...");

    Ok(())
}
