use anyhow::{Context, Result};
use genproto::merchant::merchant_service_server::MerchantServiceServer;
use merchant::{config::ServerConfig, service::MerchantServiceImpl, state::AppState};
use shared::{
    config::{Config, ConnectionManager},
    utils::{Telemetry, init_logger},
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let (server_config, state, telemetry) = setup().await.context("Failed to setup application")?;

    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);

    let server_handle = run_servers(server_config, state, shutdown_tx.clone())
        .await
        .context("Failed to start servers")?;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Shutdown signal received (Ctrl+C).");
        }
        _ = shutdown_rx.recv() => {
            info!("🛑 Shutdown signal received from internal component.");
        }
    }

    shutdown(telemetry, server_handle).await;

    Ok(())
}

async fn setup() -> Result<(ServerConfig, Arc<AppState>, Telemetry)> {
    dotenv::dotenv().ok();

    let is_dev = std::env::var("DEV_MODE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    let is_enable_file = std::env::var("ENABLE_FILE_LOG")
        .map(|v| v == "true")
        .unwrap_or(false);

    let config = Config::init().context("Failed to load configuration")?;
    let server_config = ServerConfig::from_config(&config)?;

    let telemetry = Telemetry::new("merchant-service", "http://otel-collector:4317".to_string());
    let logger_provider = telemetry.init_logger();
    let _meter_provider = telemetry.init_meter();
    let _tracer_provider = telemetry.init_tracer();

    init_logger(
        logger_provider.clone(),
        "merchant-service",
        is_dev,
        is_enable_file,
    );

    let db_pool = ConnectionManager::new_pool(
        &server_config.database_url,
        server_config.run_migrations,
        config.db_min_conn,
        config.db_max_conn,
    )
    .await
    .context("Failed to initialize database pool")?;

    let state = Arc::new(
        AppState::new(db_pool)
            .await
            .context("Failed to create AppState")?,
    );

    info!("✅ Application setup completed successfully.");
    Ok((server_config, state, telemetry))
}

async fn run_servers(
    server_config: ServerConfig,
    state: Arc<AppState>,
    shutdown_tx: broadcast::Sender<()>,
) -> Result<tokio::task::JoinHandle<()>> {
    let service = MerchantServiceImpl::new(state.di_container.clone());
    let grpc_addr = server_config.grpc_addr;

    let shutdown_tx_for_server = shutdown_tx.clone();
    let shutdown_tx_for_ctrlc = shutdown_tx.clone();

    let server_handle = tokio::spawn(async move {
        loop {
            info!("Attempting to start gRPC server on {grpc_addr}");

            let shutdown_rx = shutdown_tx_for_server.subscribe();

            match start_grpc_server(service.clone(), grpc_addr, shutdown_rx).await {
                Ok(()) => {
                    info!("gRPC server stopped gracefully.");
                    break;
                }
                Err(e) => {
                    error!("gRPC server failed: {e}. Restarting in 5s...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    });

    tokio::spawn(async move {
        if let Err(e) = tokio::signal::ctrl_c().await {
            error!("Failed to listen for shutdown signal: {}", e);
        } else {
            info!("Ctrl+C signal detected, broadcasting shutdown...");
            if let Err(e) = shutdown_tx_for_ctrlc.send(()) {
                warn!("Failed to send shutdown signal: {}", e);
            }
        }
    });

    Ok(server_handle)
}

async fn shutdown(telemetry: Telemetry, server_handle: tokio::task::JoinHandle<()>) {
    info!("🛑 Shutting down all servers...");

    let shutdown_timeout = tokio::time::Duration::from_secs(30);
    let shutdown_result = tokio::time::timeout(shutdown_timeout, server_handle).await;

    match shutdown_result {
        Ok(join_result) => {
            if let Err(e) = join_result {
                error!("Server task panicked: {}", e);
            }
            info!("✅ All servers shutdown gracefully.");
        }
        Err(_) => {
            warn!("⚠️  Shutdown timeout reached, forcing exit.");
        }
    }

    if let Err(e) = telemetry.shutdown().await {
        error!("Failed to shutdown telemetry: {}", e);
    }

    info!("✅ Merchant Service shutdown complete.");
}

async fn start_grpc_server(
    service: MerchantServiceImpl,
    addr: std::net::SocketAddr,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    info!("Starting gRPC server on {addr}");

    let shutdown_future = async move {
        let _ = shutdown_rx.recv().await;
        info!("gRPC server received shutdown signal");
    };

    tonic::transport::Server::builder()
        .add_service(MerchantServiceServer::new(service))
        .serve_with_shutdown(addr, shutdown_future)
        .await
        .context("gRPC server failed to start or serve")
}
