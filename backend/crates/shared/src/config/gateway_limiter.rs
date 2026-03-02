use std::env;

#[derive(Debug, Clone)]
pub struct GatewayLimiterConfig {
    pub cb_max_failures: u64,
    pub cb_reset_timeout_sec: u64,
    pub rate_limit: usize,
}

impl GatewayLimiterConfig {
    pub fn from_env() -> Self {
        Self {
            cb_max_failures: env::var("GATEWAY_CB_MAX_FAILURES")
                .unwrap_or_else(|_| "50".into())
                .parse()
                .expect("invalid GATEWAY_CB_MAX_FAILURES"),

            cb_reset_timeout_sec: env::var("GATEWAY_CB_RESET_TIMEOUT_SEC")
                .unwrap_or_else(|_| "5".into())
                .parse()
                .expect("invalid GATEWAY_CB_RESET_TIMEOUT_SEC"),

            rate_limit: env::var("GATEWAY_RATE_LIMIT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("invalid GATEWAY_RATE_LIMIT"),
        }
    }
}
