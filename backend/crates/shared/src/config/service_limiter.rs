use std::env;

#[derive(Clone, Debug)]
pub struct ServiceLimiterConfig {
    pub max_concurrent: usize,
}

impl ServiceLimiterConfig {
    pub fn from_env() -> Self {
        Self {
            max_concurrent: env::var("SERVICE_MAX_CONCURRENT")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .expect("invalid SERVICE_MAX_CONCURRENT"),
        }
    }
}
