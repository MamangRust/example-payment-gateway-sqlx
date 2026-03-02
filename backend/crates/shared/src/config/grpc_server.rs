use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GrpcServerConfig {
    pub concurrency_limit: usize,
    pub max_concurrent_streams: u32,
    pub timeout_secs: u64,
    pub tcp_keepalive_secs: u64,
    pub http2_keepalive_interval_secs: u64,
    pub http2_keepalive_timeout_secs: u64,
    pub initial_connection_window_size_mb: u32,
    pub initial_stream_window_size_mb: u32,
    pub tcp_nodelay: bool,
}

impl GrpcServerConfig {
    pub fn from_env() -> Result<Self> {
        let config = Self {
            concurrency_limit: Self::get_env("GRPC_SERVER_CONCURRENCY_LIMIT").unwrap_or(3000),

            max_concurrent_streams: Self::get_env("GRPC_SERVER_MAX_CONCURRENT_STREAMS")
                .unwrap_or(2048),

            timeout_secs: Self::get_env("GRPC_SERVER_TIMEOUT_SECS").unwrap_or(15),

            tcp_keepalive_secs: Self::get_env("GRPC_SERVER_TCP_KEEPALIVE_SECS").unwrap_or(60),

            http2_keepalive_interval_secs: Self::get_env(
                "GRPC_SERVER_HTTP2_KEEPALIVE_INTERVAL_SECS",
            )
            .unwrap_or(20),

            http2_keepalive_timeout_secs: Self::get_env("GRPC_SERVER_HTTP2_KEEPALIVE_TIMEOUT_SECS")
                .unwrap_or(10),

            initial_connection_window_size_mb: Self::get_env(
                "GRPC_SERVER_INITIAL_CONNECTION_WINDOW_SIZE_MB",
            )
            .unwrap_or(32),

            initial_stream_window_size_mb: Self::get_env(
                "GRPC_SERVER_INITIAL_STREAM_WINDOW_SIZE_MB",
            )
            .unwrap_or(16),

            tcp_nodelay: Self::get_env("GRPC_SERVER_TCP_NODELAY").unwrap_or(true),
        };

        tracing::info!("gRPC Server Config loaded: {:?}", config);
        Ok(config)
    }

    fn get_env<T: std::str::FromStr>(key: &str) -> Option<T> {
        std::env::var(key).ok().and_then(|v| v.parse().ok())
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }

    pub fn tcp_keepalive(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.tcp_keepalive_secs))
    }

    pub fn http2_keepalive_interval(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.http2_keepalive_interval_secs))
    }

    pub fn http2_keepalive_timeout(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.http2_keepalive_timeout_secs))
    }

    pub fn initial_connection_window_size(&self) -> Option<u32> {
        Some(self.initial_connection_window_size_mb * 1024 * 1024)
    }

    pub fn initial_stream_window_size(&self) -> Option<u32> {
        Some(self.initial_stream_window_size_mb * 1024 * 1024)
    }
}
