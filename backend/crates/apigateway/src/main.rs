use anyhow::{Context, Result};
use apigateway::{handler::AppRouter, state::AppState};
use dotenv::dotenv;
use shared::{
    config::Config,
    utils::{Telemetry, init_logger},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let is_dev = std::env::var("DEV_MODE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let is_enable_file = std::env::var("ENABLE_FILE_LOG")
        .map(|v| v == "true")
        .unwrap_or(false);

    let telemetry = Telemetry::new("apigateway", "http://otel-collector:4317".to_string());

    let logger_provider = telemetry.init_logger();

    let _meter_provider = telemetry.init_meter();
    let _tracer_provider = telemetry.init_tracer();

    init_logger(
        logger_provider.clone(),
        "apigateway",
        is_dev,
        is_enable_file,
    );

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

    telemetry.shutdown().await?;

    Ok(())
}
