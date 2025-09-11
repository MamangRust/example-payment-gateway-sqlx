use anyhow::{Context, Result};
use genproto::transfer::transfer_service_server::TransferServiceServer;
use shared::config::{Config, ConnectionManager};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use transfer::{
    config::ServerConfig,
    service::{TransferServiceImpl, TransferStats, TransferStatsByCard},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = Config::init().context("Failed to load configuration")?;

    let server_config = ServerConfig::from_config(&config)?;

    let db_pool =
        ConnectionManager::new_pool(&server_config.database_url, server_config.run_migrations)
            .await
            .context("Failed to initialize database pool")?;

    let state = Arc::new(
        AppState::new(db_pool)
            .await
            .context("Failed to create AppState")?,
    );

    let stats = TransferStats {
        amount: state.di_container.transfer_stats.amount_service.clone(),
        status: state.di_container.transfer_stats.status_service.clone(),
    };

    let statsbycard = TransferStatsByCard {
        amount: state
            .di_container
            .transfer_stats_bycard
            .amount_service
            .clone(),
        status: state
            .di_container
            .transfer_stats_bycard
            .status_service
            .clone(),
    };

    let service = TransferServiceImpl::new(
        state.di_container.transfer_query.service.clone(),
        state.di_container.transfer_command.service.clone(),
        stats,
        statsbycard,
    );

    let (shutdown_tx, _) = broadcast::channel(1);

    let grpc_addr = server_config.grpc_addr;
    let grpc_shutdown_rx = shutdown_tx.subscribe();
    let grpc_handle = tokio::spawn(async move {
        loop {
            match start_grpc_server(service.clone(), grpc_addr, grpc_shutdown_rx.resubscribe())
                .await
            {
                Ok(()) => {
                    info!("gRPC server stopped gracefully");
                    break;
                }
                Err(e) => {
                    error!("❌ gRPC server failed: {e}. Restarting in 5s...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    });

    let signal_shutdown_tx = shutdown_tx.clone();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                info!("🛑 Shutdown signal received.");
                if let Err(e) = signal_shutdown_tx.send(()) {
                    warn!("Failed to send shutdown signal: {e:?}");
                }
            }
            Err(e) => {
                error!("Failed to listen for shutdown signal: {e:?}");
            }
        }
    });

    let mut shutdown_rx = shutdown_tx.subscribe();
    let _ = shutdown_rx.recv().await;

    info!("🛑 Shutting down all servers...");

    let shutdown_timeout = tokio::time::Duration::from_secs(30);
    let shutdown_result = tokio::time::timeout(shutdown_timeout, async {
        let _ = tokio::join!(grpc_handle);
    })
    .await;

    match shutdown_result {
        Ok(_) => info!("✅ All servers shutdown gracefully"),
        Err(_) => {
            warn!("⚠️  Shutdown timeout reached, forcing exit");
        }
    }

    info!("✅ Auth Service shutdown complete.");
    Ok(())
}

async fn start_grpc_server(
    service: TransferServiceImpl,
    addr: std::net::SocketAddr,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    info!("Starting gRPC server on {addr}");

    let shutdown_future = async move {
        let _ = shutdown_rx.recv().await;
        info!("gRPC server received shutdown signal");
    };

    tonic::transport::Server::builder()
        .add_service(TransferServiceServer::new(service))
        .serve_with_shutdown(addr, shutdown_future)
        .await
        .context("gRPC server failed to start or serve")
}
