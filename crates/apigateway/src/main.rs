use anyhow::{Context, Result};
use apigateway::{handler::AppRouter, state::AppState};
use dotenv::dotenv;
use shared::config::Config;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

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
