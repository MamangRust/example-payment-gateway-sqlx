use async_trait::async_trait;
use opentelemetry::{Context, KeyValue};
use std::sync::Arc;
use tonic::Request;

use crate::observability::{Method, TracingContext};

pub type DynTracingMetricsCore = Arc<dyn TracingMetricsCoreTrait + Send + Sync>;

#[async_trait]
pub trait TracingMetricsCoreTrait {
    fn inject_trace_context<T>(&self, cx: &Context, request: &mut Request<T>);

    fn start_tracing(&self, operation_name: &str, attributes: Vec<KeyValue>) -> TracingContext;

    async fn complete_tracing_success(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        message: &str,
    );

    async fn complete_tracing_error(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        error_message: &str,
    );

    async fn complete_tracing_internal(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        is_success: bool,
        message: &str,
    );
}
