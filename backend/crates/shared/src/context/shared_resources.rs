use std::sync::Arc;

use crate::{cache::CacheStore, observability::TracingMetrics};

pub struct SharedResources {
    pub tracing_metrics: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}
