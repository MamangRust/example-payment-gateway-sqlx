use crate::service::{
    AuthGrpcClientService, CardGrpcClientService, GrpcClients, MerchantGrpcClientService,
    RoleGrpcClientService, SaldoGrpcClientService, TopupGrpcClientService,
    TransactionGrpcClientService, TransferGrpcClientService, UserGrpcClientService,
    WithdrawGrpcClientService,
};
use anyhow::{Context, Result};
use shared::cache::CacheStore;
use shared::observability::TracingMetricsCore;
use shared::{
    abstract_trait::{
        auth::http::DynAuthGrpcClient, card::http::DynCardGrpcClientService,
        merchant::http::DynMerchantGrpcClientService, role::http::DynRoleGrpcClientService,
        saldo::http::DynSaldoGrpcClientService, topup::http::DynTopupGrpcClientService,
        transaction::http::DynTransactionGrpcClientService,
        transfer::http::DynTransferGrpcClientService, user::http::DynUserGrpcServiceClient,
        withdraw::http::DynWithdrawGrpcClientService,
    },
    context::shared_resources::SharedResources,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct DependenciesInject {
    pub auth_clients: DynAuthGrpcClient,
    pub card_clients: DynCardGrpcClientService,
    pub merchant_clients: DynMerchantGrpcClientService,
    pub role_clients: DynRoleGrpcClientService,
    pub saldo_clients: DynSaldoGrpcClientService,
    pub topup_clients: DynTopupGrpcClientService,
    pub transaction_clients: DynTransactionGrpcClientService,
    pub transfer_clients: DynTransferGrpcClientService,
    pub user_clients: DynUserGrpcServiceClient,
    pub withdraw_clients: DynWithdrawGrpcClientService,
    pub cache_store: Arc<CacheStore>,
}

impl std::fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("auth_service", &"DynAuthService")
            .field("card_service", &"CardService")
            .field("merchant_service", &"MerchantService")
            .field("role_service", &"RoleService")
            .field("saldo_service", &"SaldoService")
            .field("topup_service", &"TopupService")
            .field("transaction_service", &"TransactionService")
            .field("transfer_service", &"TransferService")
            .field("user_service", &"UserService")
            .field("withdraw_service", &"WithdrawService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(clients: GrpcClients, cache_store: Arc<CacheStore>) -> Result<Self> {
        let tracing_metrics =
            Arc::new(TracingMetricsCore::new("apigateway").context("failed initialize tracing")?);

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let auth_clients: DynAuthGrpcClient = Arc::new(
            AuthGrpcClientService::new(clients.auth, &shared)
                .context("failed initialize auth grpc service")?,
        ) as DynAuthGrpcClient;

        let card_clients = Arc::new(
            CardGrpcClientService::new(clients.card, &shared)
                .context("failed initialize card grpc service")?,
        ) as DynCardGrpcClientService;

        let merchant_clients = Arc::new(
            MerchantGrpcClientService::new(clients.merchant, &shared)
                .context("failed initialize merchant grpc service")?,
        ) as DynMerchantGrpcClientService;

        let role_clients = Arc::new(
            RoleGrpcClientService::new(clients.role, &shared)
                .context("failed initialize role grpc service")?,
        ) as DynRoleGrpcClientService;

        let saldo_clients = Arc::new(
            SaldoGrpcClientService::new(clients.saldo, &shared)
                .context("failed initialize saldo grpc service")?,
        ) as DynSaldoGrpcClientService;

        let topup_clients = Arc::new(
            TopupGrpcClientService::new(clients.topup, &shared)
                .context("failed initialize topup grpc service")?,
        ) as DynTopupGrpcClientService;

        let transaction_clients = Arc::new(
            TransactionGrpcClientService::new(clients.transaction, &shared)
                .context("failed initialize transaction grpc service")?,
        ) as DynTransactionGrpcClientService;

        let transfer_clients = Arc::new(
            TransferGrpcClientService::new(clients.transfer, &shared)
                .context("failed initialize transfer grpc service")?,
        ) as DynTransferGrpcClientService;

        let user_clients = Arc::new(
            UserGrpcClientService::new(clients.user, &shared)
                .context("failed initialize user grpc service")?,
        ) as DynUserGrpcServiceClient;

        let withdraw_clients = Arc::new(
            WithdrawGrpcClientService::new(clients.withdraw, &shared)
                .context("failed initialize withdraw grpc service")?,
        ) as DynWithdrawGrpcClientService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            auth_clients,
            card_clients,
            merchant_clients,
            role_clients,
            saldo_clients,
            topup_clients,
            transaction_clients,
            transfer_clients,
            user_clients,
            withdraw_clients,
            cache_store: shared.cache_store,
        })
    }

    fn spawn_monitoring_task(cache: Arc<CacheStore>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let ref_count = cache.get_ref_count();

                if ref_count > 500 {
                    warn!("⚠️  High cache reference count: {}", ref_count);
                } else {
                    info!("📊 Gateway cache reference count: {}", ref_count);
                }

                if let Ok(stats) = cache.get_stats().await {
                    info!(
                        "💾 Cache: {} keys, {} hit rate, {}",
                        stats.total_keys,
                        format!("{:.1}%", stats.hit_rate),
                        stats.memory_used_human
                    );
                }
            }
        });
    }

    fn spawn_cleanup_task(cache: Arc<CacheStore>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(120));
            loop {
                interval.tick().await;
                info!("🧹 Running periodic gateway cache cleanup...");

                match cache.clear_expired().await {
                    Ok(scanned) => {
                        info!("✅ Cleanup scanned {} keys", scanned);
                    }
                    Err(e) => {
                        error!("❌ Cleanup failed: {}", e);
                    }
                }

                if let Ok(stats) = cache.get_stats().await {
                    info!(
                        "📊 After cleanup: {} keys, {} memory",
                        stats.total_keys, stats.memory_used_human
                    );
                }

                let ref_count = cache.get_ref_count();
                info!("✅ Cleanup completed. Ref count: {}", ref_count);
            }
        });
    }
}
