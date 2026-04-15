use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GrpcClientConfig {
    pub pool_size: usize,
    pub concurrency_per_connection: usize,
    pub rate_limit_per_sec: u64,
    pub connect_timeout_secs: u64,
    pub request_timeout_secs: u64,
    pub tcp_keepalive_secs: u64,
    pub keepalive_timeout_secs: u64,
    pub http2_keepalive_interval_secs: u64,
    pub initial_connection_window_size_mb: u32,
    pub initial_stream_window_size_mb: u32,
    pub tcp_nodelay: bool,
    pub keep_alive_while_idle: bool,
}

impl GrpcClientConfig {
    pub fn from_env() -> Result<Self> {
        let config = Self {
            pool_size: Self::get_env("GRPC_CLIENT_POOL_SIZE").unwrap_or(25),

            concurrency_per_connection: Self::get_env("GRPC_CLIENT_CONCURRENCY_PER_CONNECTION")
                .unwrap_or(500),

            rate_limit_per_sec: Self::get_env("GRPC_CLIENT_RATE_LIMIT_PER_SEC").unwrap_or(2000),

            connect_timeout_secs: Self::get_env("GRPC_CLIENT_CONNECT_TIMEOUT_SECS").unwrap_or(5),

            request_timeout_secs: Self::get_env("GRPC_CLIENT_REQUEST_TIMEOUT_SECS").unwrap_or(15),

            tcp_keepalive_secs: Self::get_env("GRPC_CLIENT_TCP_KEEPALIVE_SECS").unwrap_or(60),

            keepalive_timeout_secs: Self::get_env("GRPC_CLIENT_KEEPALIVE_TIMEOUT_SECS")
                .unwrap_or(20),

            http2_keepalive_interval_secs: Self::get_env(
                "GRPC_CLIENT_HTTP2_KEEPALIVE_INTERVAL_SECS",
            )
            .unwrap_or(30),

            initial_connection_window_size_mb: Self::get_env(
                "GRPC_CLIENT_INITIAL_CONNECTION_WINDOW_SIZE_MB",
            )
            .unwrap_or(4),

            initial_stream_window_size_mb: Self::get_env(
                "GRPC_CLIENT_INITIAL_STREAM_WINDOW_SIZE_MB",
            )
            .unwrap_or(2),

            tcp_nodelay: Self::get_env("GRPC_CLIENT_TCP_NODELAY").unwrap_or(true),

            keep_alive_while_idle: Self::get_env("GRPC_CLIENT_KEEP_ALIVE_WHILE_IDLE")
                .unwrap_or(true),
        };

        tracing::info!("gRPC Client Config loaded: {:?}", config);
        Ok(config)
    }

    fn get_env<T: std::str::FromStr>(key: &str) -> Option<T> {
        std::env::var(key).ok().and_then(|v| v.parse().ok())
    }

    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
    }

    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_secs)
    }

    pub fn tcp_keepalive(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.tcp_keepalive_secs))
    }

    pub fn keepalive_timeout(&self) -> Duration {
        Duration::from_secs(self.keepalive_timeout_secs)
    }

    pub fn http2_keepalive_interval(&self) -> Duration {
        Duration::from_secs(self.http2_keepalive_interval_secs)
    }

    pub fn rate_limit_duration(&self) -> Duration {
        Duration::from_secs(1)
    }
}
