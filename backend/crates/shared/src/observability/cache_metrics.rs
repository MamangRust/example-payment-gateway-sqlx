use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use std::sync::Arc;

pub type CacheMetrics = Arc<CacheMetricsCore>;

#[derive(Clone, Debug)]
pub struct CacheMetricsCore {
    cache_operations: Counter<u64>,
    cache_hits: Counter<u64>,
    cache_misses: Counter<u64>,
    operation_duration: Histogram<f64>,
    cache_errors: Counter<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum CacheOperation {
    Get,
    Set,
    Delete,
    Clear,
    Invalidate,
    GetStats,
}

impl CacheOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Set => "set",
            Self::Delete => "delete",
            Self::Clear => "clear",
            Self::Invalidate => "invalidate",
            Self::GetStats => "get_stats",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CacheResult {
    Hit,
    Miss,
    Success,
    Error,
}

impl CacheResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hit => "hit",
            Self::Miss => "miss",
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}

impl CacheMetricsCore {
    pub fn new(name: &'static str) -> Self {
        let meter = global::meter(name);

        let cache_operations = meter
            .u64_counter("cache_operations_total")
            .with_description("Total number of cache operations")
            .build();

        let cache_hits = meter
            .u64_counter("cache_hits_total")
            .with_description("Total number of cache hits")
            .build();

        let cache_misses = meter
            .u64_counter("cache_misses_total")
            .with_description("Total number of cache misses")
            .build();

        let operation_duration = meter
            .f64_histogram("cache_operation_duration_seconds")
            .with_description("Cache operation duration in seconds")
            .with_unit("s")
            .build();

        let cache_errors = meter
            .u64_counter("cache_errors_total")
            .with_description("Total number of cache errors")
            .build();

        Self {
            cache_operations,
            cache_hits,
            cache_misses,
            operation_duration,
            cache_errors,
        }
    }

    pub fn record_operation(
        &self,
        operation: CacheOperation,
        result: CacheResult,
        duration_secs: f64,
    ) {
        let attributes = &[
            KeyValue::new("cache.operation", operation.as_str()),
            KeyValue::new("cache.result", result.as_str()),
        ];

        self.cache_operations.add(1, attributes);

        self.operation_duration.record(duration_secs, attributes);

        match result {
            CacheResult::Hit => {
                self.cache_hits
                    .add(1, &[KeyValue::new("cache.operation", operation.as_str())]);
            }
            CacheResult::Miss => {
                self.cache_misses
                    .add(1, &[KeyValue::new("cache.operation", operation.as_str())]);
            }
            CacheResult::Error => {
                self.cache_errors
                    .add(1, &[KeyValue::new("cache.operation", operation.as_str())]);
            }
            CacheResult::Success => {}
        }
    }

    pub fn record_hit(&self, operation: CacheOperation, duration_secs: f64) {
        self.record_operation(operation, CacheResult::Hit, duration_secs);
    }

    pub fn record_miss(&self, operation: CacheOperation, duration_secs: f64) {
        self.record_operation(operation, CacheResult::Miss, duration_secs);
    }

    pub fn record_success(&self, operation: CacheOperation, duration_secs: f64) {
        self.record_operation(operation, CacheResult::Success, duration_secs);
    }

    pub fn record_error(&self, operation: CacheOperation, duration_secs: f64) {
        self.record_operation(operation, CacheResult::Error, duration_secs);
    }
}

impl Default for CacheMetricsCore {
    fn default() -> Self {
        Self::new("cache")
    }
}
